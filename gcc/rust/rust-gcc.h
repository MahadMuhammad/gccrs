// rust-gcc.cc -- Rust frontend to gcc IR.
// Copyright (C) 2011-2022 Free Software Foundation, Inc.
// Contributed by Ian Lance Taylor, Google.
// forked from gccgo

// This file is part of GCC.

// GCC is free software; you can redistribute it and/or modify it under
// the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 3, or (at your option) any later
// version.

// GCC is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
// for more details.

// You should have received a copy of the GNU General Public License
// along with GCC; see the file COPYING3.  If not see
// <http://www.gnu.org/licenses/>.

#include "rust-system.h"

// This has to be included outside of extern "C", so we have to
// include it here before tree.h includes it later.
#include <gmp.h>

#include "tree.h"
#include "rust-location.h"

// TODO: this will have to be significantly modified to work with Rust

// Bvariable is a bit more complicated, because of zero-sized types.
// The GNU linker does not permit dynamic variables with zero size.
// When we see such a variable, we generate a version of the type with
// non-zero size.  However, when referring to the global variable, we
// want an expression of zero size; otherwise, if, say, the global
// variable is passed to a function, we will be passing a
// non-zero-sized value to a zero-sized value, which can lead to a
// miscompilation.

class Bvariable
{
public:
  Bvariable (tree t) : t_ (t), orig_type_ (NULL) {}

  Bvariable (tree t, tree orig_type) : t_ (t), orig_type_ (orig_type) {}

  // Get the tree for use as an expression.
  tree get_tree (Location) const;

  // Get the actual decl;
  tree get_decl () const { return this->t_; }

private:
  tree t_;
  tree orig_type_;
};

// This file implements the interface between the Rust frontend proper
// and the gcc IR.  This implements specific instantiations of
// abstract classes defined by the Rust frontend proper.  The Rust
// frontend proper class methods of these classes to generate the
// backend representation.

class Gcc_backend : public Backend
{
public:
  Gcc_backend ();

  void debug (tree t);
  void debug (Bvariable *t);

  tree get_identifier_node (const std::string &str);

  // Types.

  tree unit_type ()
  {
    static tree unit_type;
    if (unit_type == nullptr)
      {
	auto unit_type_node = struct_type ({});
	unit_type = named_type ("()", unit_type_node,
				::Linemap::predeclared_location ());
      }

    return unit_type;
  }

  tree bool_type () { return boolean_type_node; }

  tree char_type () { return char_type_node; }

  tree wchar_type ();

  int get_pointer_size ();

  tree raw_str_type ();

  tree integer_type (bool, int);

  tree float_type (int);

  tree complex_type (int);

  tree pointer_type (tree);

  tree reference_type (tree);

  tree immutable_type (tree);

  tree function_type (const typed_identifier &,
		      const std::vector<typed_identifier> &,
		      const std::vector<typed_identifier> &, tree,
		      const Location);

  tree function_type_varadic (const typed_identifier &,
			      const std::vector<typed_identifier> &,
			      const std::vector<typed_identifier> &, tree,
			      const Location);

  tree function_ptr_type (tree, const std::vector<tree> &, Location);

  tree struct_type (const std::vector<typed_identifier> &);

  tree union_type (const std::vector<typed_identifier> &);

  tree array_type (tree, tree);

  tree named_type (const std::string &, tree, Location);

  int64_t type_size (tree);

  int64_t type_alignment (tree);

  int64_t type_field_alignment (tree);

  int64_t type_field_offset (tree, size_t index);

  // Expressions.

  tree zero_expression (tree);

  tree unit_expression () { return integer_zero_node; }

  tree var_expression (Bvariable *var, Location);

  tree integer_constant_expression (tree type, mpz_t val);

  tree float_constant_expression (tree type, mpfr_t val);

  tree complex_constant_expression (tree type, mpc_t val);

  tree string_constant_expression (const std::string &val);

  tree wchar_constant_expression (wchar_t c);

  tree char_constant_expression (char c);

  tree boolean_constant_expression (bool val);

  tree real_part_expression (tree bcomplex, Location);

  tree imag_part_expression (tree bcomplex, Location);

  tree complex_expression (tree breal, tree bimag, Location);

  tree convert_expression (tree type, tree expr, Location);

  tree struct_field_expression (tree, size_t, Location);

  tree compound_expression (tree, tree, Location);

  tree conditional_expression (tree, tree, tree, tree, tree, Location);

  tree negation_expression (NegationOperator op, tree expr, Location);

  tree arithmetic_or_logical_expression (ArithmeticOrLogicalOperator op,
					 tree left, tree right, Location);

  tree arithmetic_or_logical_expression_checked (ArithmeticOrLogicalOperator op,
						 tree left, tree right,
						 Location, Bvariable *receiver);

  tree comparison_expression (ComparisonOperator op, tree left, tree right,
			      Location);

  tree lazy_boolean_expression (LazyBooleanOperator op, tree left, tree right,
				Location);

  tree constructor_expression (tree, bool, const std::vector<tree> &, int,
			       Location);

  tree array_constructor_expression (tree, const std::vector<unsigned long> &,
				     const std::vector<tree> &, Location);

  tree array_initializer (tree, tree, tree, tree, tree, tree *, Location);

  tree array_index_expression (tree array, tree index, Location);

  tree call_expression (tree fn, const std::vector<tree> &args,
			tree static_chain, Location);

  // Statements.

  tree init_statement (tree, Bvariable *var, tree init);

  tree assignment_statement (tree lhs, tree rhs, Location);

  tree return_statement (tree, const std::vector<tree> &, Location);

  tree if_statement (tree, tree condition, tree then_block, tree else_block,
		     Location);

  tree compound_statement (tree, tree);

  tree statement_list (const std::vector<tree> &);

  tree exception_handler_statement (tree bstat, tree except_stmt,
				    tree finally_stmt, Location);

  tree loop_expression (tree body, Location);

  tree exit_expression (tree condition, Location);

  // Blocks.

  tree block (tree, tree, const std::vector<Bvariable *> &, Location, Location);

  void block_add_statements (tree, const std::vector<tree> &);

  // Variables.

  Bvariable *error_variable () { return new Bvariable (error_mark_node); }

  Bvariable *global_variable (const std::string &var_name,
			      const std::string &asm_name, tree type,
			      bool is_external, bool is_hidden,
			      bool in_unique_section, Location location);

  void global_variable_set_init (Bvariable *, tree);

  Bvariable *local_variable (tree, const std::string &, tree, Bvariable *,
			     Location);

  Bvariable *parameter_variable (tree, const std::string &, tree, Location);

  Bvariable *static_chain_variable (tree, const std::string &, tree, Location);

  Bvariable *temporary_variable (tree, tree, tree, tree, bool, Location,
				 tree *);

  // Labels.

  tree label (tree, const std::string &name, Location);

  tree label_definition_statement (tree);

  tree goto_statement (tree, Location);

  tree label_address (tree, Location);

  // Functions.

  tree function (tree fntype, const std::string &name,
		 const std::string &asm_name, unsigned int flags, Location);

  tree function_defer_statement (tree function, tree undefer, tree defer,
				 Location);

  bool function_set_parameters (tree function,
				const std::vector<Bvariable *> &);

  void write_global_definitions (const std::vector<tree> &,
				 const std::vector<tree> &,
				 const std::vector<tree> &,
				 const std::vector<Bvariable *> &);

  void write_export_data (const char *bytes, unsigned int size);

private:
  tree fill_in_fields (tree, const std::vector<typed_identifier> &);

  tree fill_in_array (tree, tree, tree);

  tree non_zero_size_type (tree);

  tree convert_tree (tree, tree, Location);
};