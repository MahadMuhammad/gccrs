// Core algorithmic facilities -*- C++ -*-

// Copyright (C) 2020-2025 Free Software Foundation, Inc.
//
// This file is part of the GNU ISO C++ Library.  This library is free
// software; you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the
// Free Software Foundation; either version 3, or (at your option)
// any later version.

// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// Under Section 7 of GPL version 3, you are granted additional
// permissions described in the GCC Runtime Library Exception, version
// 3.1, as published by the Free Software Foundation.

// You should have received a copy of the GNU General Public License and
// a copy of the GCC Runtime Library Exception along with this program;
// see the files COPYING3 and COPYING.RUNTIME respectively.  If not, see
// <http://www.gnu.org/licenses/>.

/** @file bits/ranges_algo.h
 *  This is an internal header file, included by other library headers.
 *  Do not attempt to use it directly. @headername{algorithm}
 */

#ifndef _RANGES_ALGO_H
#define _RANGES_ALGO_H 1

#if __cplusplus > 201703L

#include <bit> // __bit_width
#if __cplusplus > 202002L
#include <optional>
#endif
#include <bits/ranges_algobase.h>
#include <bits/ranges_util.h>
#include <bits/uniform_int_dist.h> // concept uniform_random_bit_generator

#if __glibcxx_concepts
namespace std _GLIBCXX_VISIBILITY(default)
{
_GLIBCXX_BEGIN_NAMESPACE_VERSION
namespace ranges
{
  namespace __detail
  {
    template<typename _Fp>
      using __by_ref_or_value_fn
	= __conditional_t<is_scalar_v<_Fp> || is_empty_v<_Fp>, _Fp, _Fp&>;

    template<typename _Comp, typename _Proj>
      struct _Comp_proj
      {
	[[no_unique_address]] __by_ref_or_value_fn<_Comp> _M_comp;
	[[no_unique_address]] __by_ref_or_value_fn<_Proj> _M_proj;

	constexpr
	_Comp_proj(_Comp& __comp, _Proj& __proj)
	: _M_comp(__comp), _M_proj(__proj)
	{ }

	template<typename _Tp, typename _Up>
	  constexpr bool
	  operator()(_Tp&& __x, _Up&& __y)
	  {
	    return std::__invoke(_M_comp,
				 std::__invoke(_M_proj, std::forward<_Tp>(__x)),
				 std::__invoke(_M_proj, std::forward<_Up>(__y)));
	  }
      };

    template<typename _Comp, typename _Proj>
      constexpr _Comp_proj<_Comp, _Proj>
      __make_comp_proj(_Comp& __comp, _Proj& __proj)
      { return {__comp, __proj}; }

    template<typename _Pred, typename _Proj>
      struct _Pred_proj
      {
	[[no_unique_address]] __by_ref_or_value_fn<_Pred> _M_pred;
	[[no_unique_address]] __by_ref_or_value_fn<_Proj> _M_proj;

	constexpr
	_Pred_proj(_Pred& __pred, _Proj& __proj)
	: _M_pred(__pred), _M_proj(__proj)
	{ }

	template<typename _Tp>
	  constexpr bool
	  operator()(_Tp&& __x)
	  {
	    return std::__invoke(_M_pred,
				 std::__invoke(_M_proj, std::forward<_Tp>(__x)));
	  }
      };

    template<typename _Pred, typename _Proj>
      constexpr _Pred_proj<_Pred, _Proj>
      __make_pred_proj(_Pred& __pred, _Proj& __proj)
      { return {__pred, __proj}; }
  } // namespace __detail

  struct __all_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!(bool)std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __all_of_fn all_of{};

  struct __any_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return true;
	return false;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __any_of_fn any_of{};

  struct __none_of_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __none_of_fn none_of{};

  template<typename _Iter, typename _Fp>
    struct in_fun_result
    {
      [[no_unique_address]] _Iter in;
      [[no_unique_address]] _Fp fun;

      template<typename _Iter2, typename _F2p>
	requires convertible_to<const _Iter&, _Iter2>
	  && convertible_to<const _Fp&, _F2p>
	constexpr
	operator in_fun_result<_Iter2, _F2p>() const &
	{ return {in, fun}; }

      template<typename _Iter2, typename _F2p>
	requires convertible_to<_Iter, _Iter2> && convertible_to<_Fp, _F2p>
	constexpr
	operator in_fun_result<_Iter2, _F2p>() &&
	{ return {std::move(in), std::move(fun)}; }
    };

  template<typename _Iter, typename _Fp>
    using for_each_result = in_fun_result<_Iter, _Fp>;

  struct __for_each_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirectly_unary_invocable<projected<_Iter, _Proj>> _Fun>
      constexpr for_each_result<_Iter, _Fun>
      operator()(_Iter __first, _Sent __last, _Fun __f, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  std::__invoke(__f, std::__invoke(__proj, *__first));
	return { std::move(__first), std::move(__f) };
      }

    template<input_range _Range, typename _Proj = identity,
	     indirectly_unary_invocable<projected<iterator_t<_Range>, _Proj>>
	       _Fun>
      constexpr for_each_result<borrowed_iterator_t<_Range>, _Fun>
      operator()(_Range&& __r, _Fun __f, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__f), std::move(__proj));
      }
  };

  inline constexpr __for_each_fn for_each{};

  template<typename _Iter, typename _Fp>
    using for_each_n_result = in_fun_result<_Iter, _Fp>;

  struct __for_each_n_fn
  {
    template<input_iterator _Iter, typename _Proj = identity,
	     indirectly_unary_invocable<projected<_Iter, _Proj>> _Fun>
      constexpr for_each_n_result<_Iter, _Fun>
      operator()(_Iter __first, iter_difference_t<_Iter> __n,
		 _Fun __f, _Proj __proj = {}) const
      {
	if constexpr (random_access_iterator<_Iter>)
	  {
	    if (__n <= 0)
	      return {std::move(__first), std::move(__f)};
	    auto __last = __first + __n;
	    return ranges::for_each(std::move(__first), std::move(__last),
				    std::move(__f), std::move(__proj));
	  }
	else
	  {
	    while (__n-- > 0)
	      {
		std::__invoke(__f, std::__invoke(__proj, *__first));
		++__first;
	      }
	    return {std::move(__first), std::move(__f)};
	  }
      }
  };

  inline constexpr __for_each_n_fn for_each_n{};

  // find, find_if and find_if_not are defined in <bits/ranges_util.h>.

  struct __find_first_of_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr _Iter1
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	for (; __first1 != __last1; ++__first1)
	  for (auto __iter = __first2; __iter != __last2; ++__iter)
	    if (std::__invoke(__pred,
			      std::__invoke(__proj1, *__first1),
			      std::__invoke(__proj2, *__iter)))
	      return __first1;
	return __first1;
      }

    template<input_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr borrowed_iterator_t<_Range1>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __find_first_of_fn find_first_of{};

  struct __count_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>,
					 const _Tp*>
      constexpr iter_difference_t<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = 0;
	for (; __first != __last; ++__first)
	  if (std::__invoke(__proj, *__first) == __value)
	    ++__n;
	return __n;
      }

    template<input_range _Range, typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<iterator_t<_Range>, _Proj>,
					 const _Tp*>
      constexpr range_difference_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__proj));
      }
  };

  inline constexpr __count_fn count{};

  struct __count_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr iter_difference_t<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = 0;
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    ++__n;
	return __n;
      }

    template<input_range _Range,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr range_difference_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __count_if_fn count_if{};

  // in_in_result, mismatch and search are defined in <bits/ranges_util.h>.

  struct __search_n_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Pred = ranges::equal_to, typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirectly_comparable<_Iter, const _Tp*, _Pred, _Proj>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, iter_difference_t<_Iter> __count,
		 const _Tp& __value, _Pred __pred = {}, _Proj __proj = {}) const
      {
	if (__count <= 0)
	  return {__first, __first};

	auto __value_comp = [&] <typename _Rp> (_Rp&& __arg) -> bool {
	    return std::__invoke(__pred, std::forward<_Rp>(__arg), __value);
	};
	if (__count == 1)
	  {
	    __first = ranges::find_if(std::move(__first), __last,
				      std::move(__value_comp),
				      std::move(__proj));
	    if (__first == __last)
	      return {__first, __first};
	    else
	      {
		auto __end = __first;
		return {__first, ++__end};
	      }
	  }

	if constexpr (sized_sentinel_for<_Sent, _Iter>
		      && random_access_iterator<_Iter>)
	  {
	    auto __tail_size = __last - __first;
	    auto __remainder = __count;

	    while (__remainder <= __tail_size)
	      {
		__first += __remainder;
		__tail_size -= __remainder;
		auto __backtrack = __first;
		while (__value_comp(std::__invoke(__proj, *--__backtrack)))
		  {
		    if (--__remainder == 0)
		      return {__first - __count, __first};
		  }
		__remainder = __count + 1 - (__first - __backtrack);
	      }
	    auto __i = __first + __tail_size;
	    return {__i, __i};
	  }
	else
	  {
	    __first = ranges::find_if(__first, __last, __value_comp, __proj);
	    while (__first != __last)
	      {
		auto __n = __count;
		auto __i = __first;
		++__i;
		while (__i != __last && __n != 1
		       && __value_comp(std::__invoke(__proj, *__i)))
		  {
		    ++__i;
		    --__n;
		  }
		if (__n == 1)
		  return {__first, __i};
		if (__i == __last)
		  return {__i, __i};
		__first = ranges::find_if(++__i, __last, __value_comp, __proj);
	      }
	    return {__first, __first};
	  }
      }

    template<forward_range _Range,
	     typename _Pred = ranges::equal_to, typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires indirectly_comparable<iterator_t<_Range>, const _Tp*,
				     _Pred, _Proj>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, range_difference_t<_Range> __count,
	       const _Tp& __value, _Pred __pred = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__count), __value,
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __search_n_fn search_n{};

#if __glibcxx_ranges_starts_ends_with // C++ >= 23
  struct __starts_with_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	iter_difference_t<_Iter1> __n1 = -1;
	iter_difference_t<_Iter2> __n2 = -1;
	if constexpr (sized_sentinel_for<_Sent1, _Iter1>)
	  __n1 = __last1 - __first1;
	if constexpr (sized_sentinel_for<_Sent2, _Iter2>)
	  __n2 = __last2 - __first2;
	return _S_impl(std::move(__first1), __last1, __n1,
		       std::move(__first2), __last2, __n2,
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	range_difference_t<_Range1> __n1 = -1;
	range_difference_t<_Range2> __n2 = -1;
	if constexpr (sized_range<_Range1>)
	  __n1 = ranges::size(__r1);
	if constexpr (sized_range<_Range2>)
	  __n2 = ranges::size(__r2);
	return _S_impl(ranges::begin(__r1), ranges::end(__r1), __n1,
		       ranges::begin(__r2), ranges::end(__r2), __n2,
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }

  private:
    template<typename _Iter1, typename _Sent1, typename _Iter2, typename _Sent2,
	     typename _Pred,
	     typename _Proj1, typename _Proj2>
      static constexpr bool
      _S_impl(_Iter1 __first1, _Sent1 __last1, iter_difference_t<_Iter1> __n1,
	      _Iter2 __first2, _Sent2 __last2, iter_difference_t<_Iter2> __n2,
	      _Pred __pred, _Proj1 __proj1, _Proj2 __proj2)
      {
	if (__first2 == __last2) [[unlikely]]
	  return true;
	else if (__n1 == -1 || __n2 == -1)
	  return ranges::mismatch(std::move(__first1), __last1,
				  std::move(__first2), __last2,
				  std::move(__pred),
				  std::move(__proj1), std::move(__proj2)).in2 == __last2;
	else if (__n1 < __n2)
	  return false;
	else if constexpr (random_access_iterator<_Iter1>)
	  return ranges::equal(__first1, __first1 + iter_difference_t<_Iter1>(__n2),
			       std::move(__first2), __last2,
			       std::move(__pred),
			       std::move(__proj1), std::move(__proj2));
	else
	  return ranges::equal(counted_iterator(std::move(__first1),
						iter_difference_t<_Iter1>(__n2)),
			       default_sentinel,
			       std::move(__first2), __last2,
			       std::move(__pred),
			       std::move(__proj1), std::move(__proj2));
      }

    friend struct __ends_with_fn;
  };

  inline constexpr __starts_with_fn starts_with{};

  struct __ends_with_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires (forward_iterator<_Iter1> || sized_sentinel_for<_Sent1, _Iter1>)
	&& (forward_iterator<_Iter2> || sized_sentinel_for<_Sent2, _Iter2>)
	&& indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	iter_difference_t<_Iter1> __n1 = -1;
	iter_difference_t<_Iter2> __n2 = -1;
	if constexpr (sized_sentinel_for<_Sent1, _Iter1>)
	  __n1 = __last1 - __first1;
	if constexpr (sized_sentinel_for<_Sent2, _Iter2>)
	  __n2 = __last2 - __first2;
	return _S_impl(std::move(__first1), __last1, __n1,
		       std::move(__first2), __last2, __n2,
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires (forward_range<_Range1> || sized_range<_Range1>)
	&& (forward_range<_Range2> || sized_range<_Range2>)
	&& indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				 _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	range_difference_t<_Range1> __n1 = -1;
	range_difference_t<_Range2> __n2 = -1;
	if constexpr (sized_range<_Range1>)
	  __n1 = ranges::size(__r1);
	if constexpr (sized_range<_Range2>)
	  __n2 = ranges::size(__r2);
	return _S_impl(ranges::begin(__r1), ranges::end(__r1), __n1,
		       ranges::begin(__r2), ranges::end(__r2), __n2,
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }

  private:
    template<typename _Iter1, typename _Sent1,
	     typename _Iter2, typename _Sent2,
	     typename _Pred,
	     typename _Proj1, typename _Proj2>
      static constexpr bool
      _S_impl(_Iter1 __first1, _Sent1 __last1, iter_difference_t<_Iter1> __n1,
	      _Iter2 __first2, _Sent2 __last2, iter_difference_t<_Iter2> __n2,
	      _Pred __pred, _Proj1 __proj1, _Proj2 __proj2)
      {
	if constexpr (!random_access_iterator<_Iter1>
		      && bidirectional_iterator<_Iter1> && same_as<_Iter1, _Sent1>
		      && bidirectional_iterator<_Iter2> && same_as<_Iter2, _Sent2>)
	  return starts_with._S_impl(std::make_reverse_iterator(__last1),
				     std::make_reverse_iterator(__first1),
				     __n1,
				     std::make_reverse_iterator(__last2),
				     std::make_reverse_iterator(__first2),
				     __n2,
				     std::move(__pred),
				     std::move(__proj1), std::move(__proj2));

	if (__first2 == __last2) [[unlikely]]
	  return true;

	if constexpr (forward_iterator<_Iter2>)
	  if (__n2 == -1)
	    __n2 = ranges::distance(__first2, __last2);

	// __glibcxx_assert(__n2 != -1);

	if (__n1 != -1)
	  {
	    if (__n1 < __n2)
	      return false;
	    auto __shift = __n1 - iter_difference_t<_Iter1>(__n2);
	    if (random_access_iterator<_Iter1>
		|| !bidirectional_iterator<_Iter1>
		|| !same_as<_Iter1, _Sent1>
		|| __shift < __n2)
	      {
		ranges::advance(__first1, __shift);
		return ranges::equal(std::move(__first1), __last1,
				     std::move(__first2), __last2,
				     std::move(__pred),
				     std::move(__proj1), std::move(__proj2));
	      }
	  }

	if constexpr (bidirectional_iterator<_Iter1> && same_as<_Iter1, _Sent1>)
	  {
	    _Iter1 __it1 = __last1;
	    if (__n1 != -1)
	      ranges::advance(__it1, -iter_difference_t<_Iter1>(__n2));
	    else
	      {
		// We can't use ranges::advance if the haystack size is
		// unknown, since we need to detect and return false if
		// it's smaller than the needle.
		iter_difference_t<_Iter2> __m = __n2;
		while (__m != 0 && __it1 != __first1)
		  {
		    --__m;
		    --__it1;
		  }
		if (__m != 0)
		  return false;
	      }
	    return ranges::equal(__it1, __last1,
				 std::move(__first2), __last2,
				 std::move(__pred),
				 std::move(__proj1), std::move(__proj2));
	  }
	else if constexpr (forward_iterator<_Iter1>)
	  {
	    // __glibcxx_assert(__n1 == -1);
	    _Iter1 __prev_first1;
	    __n1 = 0;
	    while (true)
	      {
		iter_difference_t<_Iter2> __m = __n2;
		_Iter1 __it1 = __first1;
		while (__m != 0 && __it1 != __last1)
		  {
		    ++__n1;
		    --__m;
		    ++__it1;
		  }
		if (__m != 0)
		  {
		    // __glibcxx_assert(__it1 == __last1);
		    if (__n1 < __n2)
		      return false;
		    __first1 = ranges::next(__prev_first1,
					    iter_difference_t<_Iter1>(__n2 - __m));
		    break;
		  }
		__prev_first1 = __first1;
		__first1 = __it1;
	      }
	    return ranges::equal(__first1, __last1,
				 std::move(__first2), __last2,
				 std::move(__pred),
				 std::move(__proj1), std::move(__proj2));
	  }
	else
	  // If the haystack is non-forward then it must be sized, in which case
	  // we already returned via the __n1 != 1 case.
	  __builtin_unreachable();
      }

  };

  inline constexpr __ends_with_fn ends_with{};
#endif // __glibcxx_ranges_starts_ends_with

  struct __find_end_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr subrange<_Iter1>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if constexpr (bidirectional_iterator<_Iter1>
		      && bidirectional_iterator<_Iter2>)
	  {
	    auto __i1 = ranges::next(__first1, __last1);
	    auto __i2 = ranges::next(__first2, __last2);
	    auto __rresult
	      = ranges::search(reverse_iterator<_Iter1>{__i1},
			       reverse_iterator<_Iter1>{__first1},
			       reverse_iterator<_Iter2>{__i2},
			       reverse_iterator<_Iter2>{__first2},
			       std::move(__pred),
			       std::move(__proj1), std::move(__proj2));
	    auto __result_first = ranges::end(__rresult).base();
	    auto __result_last = ranges::begin(__rresult).base();
	    if (__result_last == __first1)
	      return {__i1, __i1};
	    else
	      return {__result_first, __result_last};
	  }
	else
	  {
	    auto __i = ranges::next(__first1, __last1);
	    if (__first2 == __last2)
	      return {__i, __i};

	    auto __result_begin = __i;
	    auto __result_end = __i;
	    for (;;)
	      {
		auto __new_range = ranges::search(__first1, __last1,
						  __first2, __last2,
						  __pred, __proj1, __proj2);
		auto __new_result_begin = ranges::begin(__new_range);
		auto __new_result_end = ranges::end(__new_range);
		if (__new_result_begin == __last1)
		  return {__result_begin, __result_end};
		else
		  {
		    __result_begin = __new_result_begin;
		    __result_end = __new_result_end;
		    __first1 = __result_begin;
		    ++__first1;
		  }
	      }
	  }
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr borrowed_subrange_t<_Range1>
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __find_end_fn find_end{};

  // adjacent_find is defined in <bits/ranges_util.h>.

  struct __is_permutation_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_equivalence_relation<projected<_Iter1, _Proj1>,
					   projected<_Iter2, _Proj2>> _Pred
	       = ranges::equal_to>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	constexpr bool __sized_iters
	  = (sized_sentinel_for<_Sent1, _Iter1>
	     && sized_sentinel_for<_Sent2, _Iter2>);
	if constexpr (__sized_iters)
	  {
	    auto __d1 = ranges::distance(__first1, __last1);
	    auto __d2 = ranges::distance(__first2, __last2);
	    if (__d1 != __d2)
	      return false;
	  }

	// Efficiently compare identical prefixes:  O(N) if sequences
	// have the same elements in the same order.
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2)
	  if (!(bool)std::__invoke(__pred,
				   std::__invoke(__proj1, *__first1),
				   std::__invoke(__proj2, *__first2)))
	      break;

	if constexpr (__sized_iters)
	  {
	    if (__first1 == __last1)
	      return true;
	  }
	else
	  {
	    auto __d1 = ranges::distance(__first1, __last1);
	    auto __d2 = ranges::distance(__first2, __last2);
	    if (__d1 == 0 && __d2 == 0)
	      return true;
	    if (__d1 != __d2)
	      return false;
	  }

	for (auto __scan = __first1; __scan != __last1; ++__scan)
	  {
	    auto&& __scan_deref = *__scan;
	    auto&& __proj_scan =
	      std::__invoke(__proj1, std::forward<decltype(__scan_deref)>(__scan_deref));
	    auto __comp_scan = [&] <typename _Tp> (_Tp&& __arg) -> bool {
	      return std::__invoke(__pred,
				   std::forward<decltype(__proj_scan)>(__proj_scan),
				   std::forward<_Tp>(__arg));
	    };
	    if (__scan != ranges::find_if(__first1, __scan,
					  __comp_scan, __proj1))
	      continue; // We've seen this one before.

	    auto __matches = ranges::count_if(__first2, __last2,
					      __comp_scan, __proj2);
	    if (__matches == 0
		|| ranges::count_if(__scan, __last1,
				    __comp_scan, __proj1) != __matches)
	      return false;
	  }
	return true;
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range1>, _Proj1>,
	       projected<iterator_t<_Range2>, _Proj2>> _Pred = ranges::equal_to>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 3560. ranges::is_permutation should short-circuit for sized_ranges
	if constexpr (sized_range<_Range1>)
	  if constexpr (sized_range<_Range2>)
	    if (ranges::distance(__r1) != ranges::distance(__r2))
	      return false;

	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __is_permutation_fn is_permutation{};

  template<typename _Iter, typename _Out>
    using copy_if_result = in_out_result<_Iter, _Out>;

  struct __copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
      constexpr copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __copy_if_fn copy_if{};

  template<typename _Iter1, typename _Iter2>
    using swap_ranges_result = in_in_result<_Iter1, _Iter2>;

  struct __swap_ranges_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2>
      requires indirectly_swappable<_Iter1, _Iter2>
      constexpr swap_ranges_result<_Iter1, _Iter2>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2) const
      {
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2)
	  ranges::iter_swap(__first1, __first2);
	return {std::move(__first1), std::move(__first2)};
      }

    template<input_range _Range1, input_range _Range2>
      requires indirectly_swappable<iterator_t<_Range1>, iterator_t<_Range2>>
      constexpr swap_ranges_result<borrowed_iterator_t<_Range1>,
				   borrowed_iterator_t<_Range2>>
      operator()(_Range1&& __r1, _Range2&& __r2) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2));
      }
  };

  inline constexpr __swap_ranges_fn swap_ranges{};

  template<typename _Iter, typename _Out>
    using unary_transform_result = in_out_result<_Iter, _Out>;

  template<typename _Iter1, typename _Iter2, typename _Out>
    struct in_in_out_result
    {
      [[no_unique_address]] _Iter1 in1;
      [[no_unique_address]] _Iter2 in2;
      [[no_unique_address]] _Out  out;

      template<typename _IIter1, typename _IIter2, typename _OOut>
	requires convertible_to<const _Iter1&, _IIter1>
	  && convertible_to<const _Iter2&, _IIter2>
	  && convertible_to<const _Out&, _OOut>
	constexpr
	operator in_in_out_result<_IIter1, _IIter2, _OOut>() const &
	{ return {in1, in2, out}; }

      template<typename _IIter1, typename _IIter2, typename _OOut>
	requires convertible_to<_Iter1, _IIter1>
	  && convertible_to<_Iter2, _IIter2>
	  && convertible_to<_Out, _OOut>
	constexpr
	operator in_in_out_result<_IIter1, _IIter2, _OOut>() &&
	{ return {std::move(in1), std::move(in2), std::move(out)}; }
    };

  template<typename _Iter1, typename _Iter2, typename _Out>
    using binary_transform_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __transform_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out,
	     copy_constructible _Fp, typename _Proj = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<_Iter, _Proj>>>
      constexpr unary_transform_result<_Iter, _Out>
      operator()(_Iter __first1, _Sent __last1, _Out __result,
		 _Fp __op, _Proj __proj = {}) const
      {
	for (; __first1 != __last1; ++__first1, (void)++__result)
	  *__result = std::__invoke(__op, std::__invoke(__proj, *__first1));
	return {std::move(__first1), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     copy_constructible _Fp, typename _Proj = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<iterator_t<_Range>, _Proj>>>
      constexpr unary_transform_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result, _Fp __op, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__op), std::move(__proj));
      }

    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, copy_constructible _Fp,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<_Iter1, _Proj1>,
				     projected<_Iter2, _Proj2>>>
      constexpr binary_transform_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Fp __binary_op,
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	for (; __first1 != __last1 && __first2 != __last2;
	     ++__first1, (void)++__first2, ++__result)
	  *__result = std::__invoke(__binary_op,
				    std::__invoke(__proj1, *__first1),
				    std::__invoke(__proj2, *__first2));
	return {std::move(__first1), std::move(__first2), std::move(__result)};
      }

    template<input_range _Range1, input_range _Range2,
	     weakly_incrementable _Out, copy_constructible _Fp,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_writable<_Out,
				   indirect_result_t<_Fp&,
				     projected<iterator_t<_Range1>, _Proj1>,
				     projected<iterator_t<_Range2>, _Proj2>>>
      constexpr binary_transform_result<borrowed_iterator_t<_Range1>,
					borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result, _Fp __binary_op,
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__binary_op),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __transform_fn transform{};

  struct __replace_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp1 _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     typename _Tp2 _GLIBCXX26_DEF_VAL_T(_Tp1)>
      requires indirectly_writable<_Iter, const _Tp2&>
	&& indirect_binary_predicate<ranges::equal_to, projected<_Iter, _Proj>,
				     const _Tp1*>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__proj, *__first) == __old_value)
	    *__first = __new_value;
	return __first;
      }

    template<input_range _Range, typename _Proj = identity,
	     typename _Tp1
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     typename _Tp2 _GLIBCXX26_DEF_VAL_T(_Tp1)>
      requires indirectly_writable<iterator_t<_Range>, const _Tp2&>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp1*>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __old_value, __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_fn replace{};

  struct __replace_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_writable<_Iter, const _Tp&>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    *__first = __new_value;
	return std::move(__first);
      }

    template<input_range _Range, typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_writable<iterator_t<_Range>, const _Tp&>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_if_fn replace_if{};

  template<typename _Iter, typename _Out>
    using replace_copy_result = in_out_result<_Iter, _Out>;

  struct __replace_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Out, typename _Proj = identity,
	     typename _Tp1 _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     typename _Tp2 _GLIBCXX26_DEF_VAL_T(iter_value_t<_Out>)>
      requires indirectly_copyable<_Iter, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<_Iter, _Proj>, const _Tp1*>
	&& output_iterator<_Out, const _Tp2&>
      constexpr replace_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first, (void)++__result)
	  if (std::__invoke(__proj, *__first) == __old_value)
	    *__result = __new_value;
	  else
	    *__result = *__first;
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, typename _Out,
	     typename _Proj = identity,
	     typename _Tp1
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     typename _Tp2 _GLIBCXX26_DEF_VAL_T(iter_value_t<_Out>)>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp1*>
	&& output_iterator<_Out, const _Tp2&>
      constexpr replace_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 const _Tp1& __old_value, const _Tp2& __new_value,
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), __old_value,
		       __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_copy_fn replace_copy{};

  template<typename _Iter, typename _Out>
    using replace_copy_if_result = in_out_result<_Iter, _Out>;

  struct __replace_copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Out,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(iter_value_t<_Out>),
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
	&& output_iterator<_Out, const _Tp&>
      constexpr replace_copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first, (void)++__result)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    *__result = __new_value;
	  else
	    *__result = *__first;
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range,
	     typename _Out,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(iter_value_t<_Out>),
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& output_iterator<_Out, const _Tp&>
      constexpr replace_copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, const _Tp& __new_value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), std::move(__pred),
		       __new_value, std::move(__proj));
      }
  };

  inline constexpr __replace_copy_if_fn replace_copy_if{};

  struct __generate_n_fn
  {
    template<input_or_output_iterator _Out, copy_constructible _Fp>
      requires invocable<_Fp&>
	&& indirectly_writable<_Out, invoke_result_t<_Fp&>>
      constexpr _Out
      operator()(_Out __first, iter_difference_t<_Out> __n, _Fp __gen) const
      {
	for (; __n > 0; --__n, (void)++__first)
	  *__first = std::__invoke(__gen);
	return __first;
      }
  };

  inline constexpr __generate_n_fn generate_n{};

  struct __generate_fn
  {
    template<input_or_output_iterator _Out, sentinel_for<_Out> _Sent,
	     copy_constructible _Fp>
      requires invocable<_Fp&>
	&& indirectly_writable<_Out, invoke_result_t<_Fp&>>
      constexpr _Out
      operator()(_Out __first, _Sent __last, _Fp __gen) const
      {
	for (; __first != __last; ++__first)
	  *__first = std::__invoke(__gen);
	return __first;
      }

    template<typename _Range, copy_constructible _Fp>
      requires invocable<_Fp&> && output_range<_Range, invoke_result_t<_Fp&>>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Fp __gen) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__gen));
      }
  };

  inline constexpr __generate_fn generate{};

  struct __remove_if_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	__first = ranges::find_if(__first, __last, __pred, __proj);
	if (__first == __last)
	  return {__first, __first};

	auto __result = __first;
	++__first;
	for (; __first != __last; ++__first)
	  if (!std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = ranges::iter_move(__first);
	      ++__result;
	    }

	return {__result, __first};
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __remove_if_fn remove_if{};

  struct __remove_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>,
					 const _Tp*>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	auto __pred = [&] (auto&& __arg) -> bool {
	  return std::forward<decltype(__arg)>(__arg) == __value;
	};
	return ranges::remove_if(__first, __last,
				 std::move(__pred), std::move(__proj));
      }

    template<forward_range _Range, typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires permutable<iterator_t<_Range>>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp*>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__proj));
      }
  };

  inline constexpr __remove_fn remove{};

  template<typename _Iter, typename _Out>
    using remove_copy_if_result = in_out_result<_Iter, _Out>;

  struct __remove_copy_if_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out>
      constexpr remove_copy_if_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr remove_copy_if_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __remove_copy_if_fn remove_copy_if{};

  template<typename _Iter, typename _Out>
    using remove_copy_result = in_out_result<_Iter, _Out>;

  struct __remove_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirectly_copyable<_Iter, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<_Iter, _Proj>,
				     const _Tp*>
      constexpr remove_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (!(std::__invoke(__proj, *__first) == __value))
	    {
	      *__result = *__first;
	      ++__result;
	    }
	return {std::move(__first), std::move(__result)};
      }

    template<input_range _Range, weakly_incrementable _Out,
	     typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& indirect_binary_predicate<ranges::equal_to,
				     projected<iterator_t<_Range>, _Proj>,
				     const _Tp*>
      constexpr remove_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 const _Tp& __value, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result), __value, std::move(__proj));
      }
  };

  inline constexpr __remove_copy_fn remove_copy{};

  struct __unique_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<_Iter, _Proj>> _Comp = ranges::equal_to>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	__first = ranges::adjacent_find(__first, __last, __comp, __proj);
	if (__first == __last)
	  return {__first, __first};

	auto __dest = __first;
	++__first;
	while (++__first != __last)
	  if (!std::__invoke(__comp,
			     std::__invoke(__proj, *__dest),
			     std::__invoke(__proj, *__first)))
	    *++__dest = ranges::iter_move(__first);
	return {++__dest, __first};
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range>, _Proj>> _Comp = ranges::equal_to>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __unique_fn unique{};

  namespace __detail
  {
    template<typename _Out, typename _Tp>
      concept __can_reread_output = input_iterator<_Out>
	&& same_as<_Tp, iter_value_t<_Out>>;
  }

  template<typename _Iter, typename _Out>
    using unique_copy_result = in_out_result<_Iter, _Out>;

  struct __unique_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<_Iter, _Proj>> _Comp = ranges::equal_to>
      requires indirectly_copyable<_Iter, _Out>
	&& (forward_iterator<_Iter>
	    || __detail::__can_reread_output<_Out, iter_value_t<_Iter>>
	    || indirectly_copyable_storable<_Iter, _Out>)
      constexpr unique_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), std::move(__result)};

	// _GLIBCXX_RESOLVE_LIB_DEFECTS
	// 4269. unique_copy passes arguments to its predicate backwards

	// TODO: perform a closer comparison with reference implementations
	if constexpr (forward_iterator<_Iter>)
	  {
	    auto __next = __first;
	    *__result = *__next;
	    while (++__next != __last)
	      if (!std::__invoke(__comp,
				 std::__invoke(__proj, *__first),
				 std::__invoke(__proj, *__next)))
		{
		  __first = __next;
		  *++__result = *__first;
		}
	    return {__next, std::move(++__result)};
	  }
	else if constexpr (__detail::__can_reread_output<_Out, iter_value_t<_Iter>>)
	  {
	    *__result = *__first;
	    while (++__first != __last)
	      if (!std::__invoke(__comp,
				 std::__invoke(__proj, *__result),
				 std::__invoke(__proj, *__first)))
		  *++__result = *__first;
	    return {std::move(__first), std::move(++__result)};
	  }
	else // indirectly_copyable_storable<_Iter, _Out>
	  {
	    auto __value = *__first;
	    *__result = __value;
	    while (++__first != __last)
	      {
		if (!(bool)std::__invoke(__comp,
					 std::__invoke(__proj, __value),
					 std::__invoke(__proj, *__first)))
		  {
		    __value = *__first;
		    *++__result = __value;
		  }
	      }
	    return {std::move(__first), std::move(++__result)};
	  }
      }

    template<input_range _Range,
	     weakly_incrementable _Out, typename _Proj = identity,
	     indirect_equivalence_relation<
	       projected<iterator_t<_Range>, _Proj>> _Comp = ranges::equal_to>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
	&& (forward_iterator<iterator_t<_Range>>
	    || __detail::__can_reread_output<_Out, range_value_t<_Range>>
	    || indirectly_copyable_storable<iterator_t<_Range>, _Out>)
      constexpr unique_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __unique_copy_fn unique_copy{};

  struct __reverse_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent>
      requires permutable<_Iter>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last) const
      {
	auto __i = ranges::next(__first, __last);
	auto __tail = __i;

	if constexpr (random_access_iterator<_Iter>)
	  {
	    if (__first != __last)
	      {
		--__tail;
		while (__first < __tail)
		  {
		    ranges::iter_swap(__first, __tail);
		    ++__first;
		    --__tail;
		  }
	      }
	    return __i;
	  }
	else
	  {
	    for (;;)
	      if (__first == __tail || __first == --__tail)
		break;
	      else
		{
		  ranges::iter_swap(__first, __tail);
		  ++__first;
		}
	    return __i;
	  }
      }

    template<bidirectional_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r));
      }
  };

  inline constexpr __reverse_fn reverse{};

  template<typename _Iter, typename _Out>
    using reverse_copy_result = in_out_result<_Iter, _Out>;

  struct __reverse_copy_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr reverse_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Sent __last, _Out __result) const
      {
	auto __i = ranges::next(__first, __last);
	auto __tail = __i;
	while (__first != __tail)
	  {
	    --__tail;
	    *__result = *__tail;
	    ++__result;
	  }
	return {__i, std::move(__result)};
      }

    template<bidirectional_range _Range, weakly_incrementable _Out>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr reverse_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, _Out __result) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__result));
      }
  };

  inline constexpr __reverse_copy_fn reverse_copy{};

  struct __rotate_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Iter __middle, _Sent __last) const
      {
	auto __lasti = ranges::next(__first, __last);
	if (__first == __middle)
	  return {__lasti, __lasti};
	if (__last == __middle)
	  return {std::move(__first), std::move(__lasti)};

	if constexpr (random_access_iterator<_Iter>)
	  {
	    auto __n = __lasti - __first;
	    auto __k = __middle - __first;

	    if (__k == __n - __k)
	      {
		ranges::swap_ranges(__first, __middle, __middle, __middle + __k);
		return {std::move(__middle), std::move(__lasti)};
	      }

	    auto __p = __first;
	    auto __ret = __first + (__lasti - __middle);

	    for (;;)
	      {
		if (__k < __n - __k)
		  {
		    // TODO: is_pod is deprecated, but this condition is
		    // consistent with the STL implementation.
		    if constexpr (__is_pod(iter_value_t<_Iter>))
		      if (__k == 1)
			{
			  auto __t = std::move(*__p);
			  ranges::move(__p + 1, __p + __n, __p);
			  *(__p + __n - 1) = std::move(__t);
			  return {std::move(__ret), std::move(__lasti)};
			}
		    auto __q = __p + __k;
		    for (decltype(__n) __i = 0; __i < __n - __k; ++ __i)
		      {
			ranges::iter_swap(__p, __q);
			++__p;
			++__q;
		      }
		    __n %= __k;
		    if (__n == 0)
		      return {std::move(__ret), std::move(__lasti)};
		    ranges::swap(__n, __k);
		    __k = __n - __k;
		  }
		else
		  {
		    __k = __n - __k;
		    // TODO: is_pod is deprecated, but this condition is
		    // consistent with the STL implementation.
		    if constexpr (__is_pod(iter_value_t<_Iter>))
		      if (__k == 1)
			{
			  auto __t = std::move(*(__p + __n - 1));
			  ranges::move_backward(__p, __p + __n - 1, __p + __n);
			  *__p = std::move(__t);
			  return {std::move(__ret), std::move(__lasti)};
			}
		    auto __q = __p + __n;
		    __p = __q - __k;
		    for (decltype(__n) __i = 0; __i < __n - __k; ++ __i)
		      {
			--__p;
			--__q;
			ranges::iter_swap(__p, __q);
		      }
		    __n %= __k;
		    if (__n == 0)
		      return {std::move(__ret), std::move(__lasti)};
		    std::swap(__n, __k);
		  }
	      }
	  }
	else if constexpr (bidirectional_iterator<_Iter>)
	  {
	    auto __tail = __lasti;

	    ranges::reverse(__first, __middle);
	    ranges::reverse(__middle, __tail);

	    while (__first != __middle && __middle != __tail)
	      {
		ranges::iter_swap(__first, --__tail);
		++__first;
	      }

	    if (__first == __middle)
	      {
		ranges::reverse(__middle, __tail);
		return {std::move(__tail), std::move(__lasti)};
	      }
	    else
	      {
		ranges::reverse(__first, __middle);
		return {std::move(__first), std::move(__lasti)};
	      }
	  }
	else
	  {
	    auto __first2 = __middle;
	    do
	      {
		ranges::iter_swap(__first, __first2);
		++__first;
		++__first2;
		if (__first == __middle)
		  __middle = __first2;
	      } while (__first2 != __last);

	    auto __ret = __first;

	    __first2 = __middle;

	    while (__first2 != __last)
	      {
		ranges::iter_swap(__first, __first2);
		++__first;
		++__first2;
		if (__first == __middle)
		  __middle = __first2;
		else if (__first2 == __last)
		  __first2 = __middle;
	      }
	    return {std::move(__ret), std::move(__lasti)};
	  }
      }

    template<forward_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r));
      }
  };

  inline constexpr __rotate_fn rotate{};

  template<typename _Iter, typename _Out>
    using rotate_copy_result = in_out_result<_Iter, _Out>;

  struct __rotate_copy_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out>
      requires indirectly_copyable<_Iter, _Out>
      constexpr rotate_copy_result<_Iter, _Out>
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Out __result) const
      {
	auto __copy1 = ranges::copy(__middle,
				    std::move(__last),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first),
				    std::move(__middle),
				    std::move(__copy1.out));
	return { std::move(__copy1.in), std::move(__copy2.out) };
      }

    template<forward_range _Range, weakly_incrementable _Out>
      requires indirectly_copyable<iterator_t<_Range>, _Out>
      constexpr rotate_copy_result<borrowed_iterator_t<_Range>, _Out>
      operator()(_Range&& __r, iterator_t<_Range> __middle, _Out __result) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r), std::move(__result));
      }
  };

  inline constexpr __rotate_copy_fn rotate_copy{};

  struct __sample_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out, typename _Gen>
      requires (forward_iterator<_Iter> || random_access_iterator<_Out>)
	&& indirectly_copyable<_Iter, _Out>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Out
      operator()(_Iter __first, _Sent __last, _Out __out,
		 iter_difference_t<_Iter> __n, _Gen&& __g) const
      {
	// FIXME: Correctly handle integer-class difference types.
	if constexpr (forward_iterator<_Iter>)
	  {
	    using _Size = iter_difference_t<_Iter>;
	    using __distrib_type = uniform_int_distribution<_Size>;
	    using __param_type = typename __distrib_type::param_type;
	    using _USize = __detail::__make_unsigned_like_t<_Size>;
	    using __uc_type
	      = common_type_t<typename remove_reference_t<_Gen>::result_type, _USize>;

	    if (__first == __last)
	      return __out;

	    __distrib_type __d{};
	    _Size __unsampled_sz = ranges::distance(__first, __last);
	    __n = std::min(__n, __unsampled_sz);

	    // If possible, we use __gen_two_uniform_ints to efficiently produce
	    // two random numbers using a single distribution invocation:

	    const __uc_type __urngrange = __g.max() - __g.min();
	    if (__urngrange / __uc_type(__unsampled_sz) >= __uc_type(__unsampled_sz))
	      // I.e. (__urngrange >= __unsampled_sz * __unsampled_sz) but without
	      // wrapping issues.
	      {
		while (__n != 0 && __unsampled_sz >= 2)
		  {
		    const pair<_Size, _Size> __p =
		      __gen_two_uniform_ints(__unsampled_sz, __unsampled_sz - 1, __g);

		    --__unsampled_sz;
		    if (__p.first < __n)
		      {
			*__out = *__first;
			++__out;
			--__n;
		      }

		    ++__first;

		    if (__n == 0) break;

		    --__unsampled_sz;
		    if (__p.second < __n)
		      {
			*__out = *__first;
			++__out;
			--__n;
		      }

		    ++__first;
		  }
	      }

	    // The loop above is otherwise equivalent to this one-at-a-time version:

	    for (; __n != 0; ++__first)
	      if (__d(__g, __param_type{0, --__unsampled_sz}) < __n)
		{
		  *__out = *__first;
		  ++__out;
		  --__n;
		}
	    return __out;
	  }
	else
	  {
	    using __distrib_type
	      = uniform_int_distribution<iter_difference_t<_Iter>>;
	    using __param_type = typename __distrib_type::param_type;
	    __distrib_type __d{};
	    iter_difference_t<_Iter> __sample_sz = 0;
	    while (__first != __last && __sample_sz != __n)
	      {
		__out[__sample_sz++] = *__first;
		++__first;
	      }
	    for (auto __pop_sz = __sample_sz; __first != __last;
		++__first, (void) ++__pop_sz)
	      {
		const auto __k = __d(__g, __param_type{0, __pop_sz});
		if (__k < __n)
		  __out[__k] = *__first;
	      }
	    return __out + iter_difference_t<_Out>(__sample_sz);
	  }
      }

    template<input_range _Range, weakly_incrementable _Out, typename _Gen>
      requires (forward_range<_Range> || random_access_iterator<_Out>)
	&& indirectly_copyable<iterator_t<_Range>, _Out>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Out
      operator()(_Range&& __r, _Out __out,
		 range_difference_t<_Range> __n, _Gen&& __g) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__out), __n,
		       std::forward<_Gen>(__g));
      }
  };

  inline constexpr __sample_fn sample{};

  struct __shuffle_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Gen>
      requires permutable<_Iter>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      _Iter
      operator()(_Iter __first, _Sent __last, _Gen&& __g) const
      {
	// FIXME: Correctly handle integer-class difference types.
	if (__first == __last)
	  return __first;

	using _DistanceType = iter_difference_t<_Iter>;
	using __ud_type = __detail::__make_unsigned_like_t<_DistanceType>;
	using __distr_type = std::uniform_int_distribution<__ud_type>;
	using __p_type = typename __distr_type::param_type;

	using __uc_type
	  = common_type_t<typename remove_reference_t<_Gen>::result_type, __ud_type>;

	const __uc_type __urngrange = __g.max() - __g.min();
	const __uc_type __urange = __uc_type(__last - __first);

	if (__urngrange / __urange >= __urange)
	  // I.e. (__urngrange >= __urange * __urange) but without wrap issues.
	  {
	    _Iter __i = __first + 1;

	    // Since we know the range isn't empty, an even number of elements
	    // means an uneven number of elements /to swap/, in which case we
	    // do the first one up front:

	    if ((__urange % 2) == 0)
	      {
		__distr_type __d{0, 1};
		ranges::iter_swap(__i++, __first + __d(__g));
	      }

	    // Now we know that __last - __i is even, so we do the rest in pairs,
	    // using a single distribution invocation to produce swap positions
	    // for two successive elements at a time:

	    while (__i != __last)
	      {
		const __uc_type __swap_range = __uc_type(__i - __first) + 1;

		const pair<__uc_type, __uc_type> __pospos =
		  __gen_two_uniform_ints(__swap_range, __swap_range + 1, __g);

		ranges::iter_swap(__i++, __first + __pospos.first);
		ranges::iter_swap(__i++, __first + __pospos.second);
	      }

	    return __i;
	  }

	__distr_type __d;

	_Iter __i = __first + 1;
	for (; __i != __last; ++__i)
	  ranges::iter_swap(__i, __first + __d(__g, __p_type(0, __i - __first)));

	return __i;
      }

    template<random_access_range _Range, typename _Gen>
      requires permutable<iterator_t<_Range>>
	&& uniform_random_bit_generator<remove_reference_t<_Gen>>
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Gen&& __g) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::forward<_Gen>(__g));
      }
  };

  inline constexpr __shuffle_fn shuffle{};

  namespace __detail
  {
    template<typename _Iter, typename _Comp>
      constexpr void
      __push_heap(_Iter __first,
		  iter_difference_t<_Iter> __holeIndex,
		  iter_difference_t<_Iter> __topIndex,
		  iter_value_t<_Iter> __value,
		  _Comp __comp)
      {
	auto __parent = (__holeIndex - 1) / 2;
	while (__holeIndex > __topIndex
	       && __comp(*(__first + __parent), __value))
	  {
	    *(__first + __holeIndex) = ranges::iter_move(__first + __parent);
	    __holeIndex = __parent;
	    __parent = (__holeIndex - 1) / 2;
	  }
	*(__first + __holeIndex) = std::move(__value);
      }
  } // namespace __detail

  struct __push_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	    __detail::__push_heap(__first, (__last - __first) - 1,
				  0, ranges::iter_move(__last - 1),
				  __comp_proj);
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __push_heap_fn push_heap{};

  namespace __detail
  {
    template<typename _Iter, typename _Comp>
      constexpr void
      __adjust_heap(_Iter __first,
		    iter_difference_t<_Iter> __holeIndex,
		    iter_difference_t<_Iter> __len,
		    iter_value_t<_Iter> __value,
		    _Comp __comp)
      {
	auto __topIndex = __holeIndex;
	auto __secondChild = __holeIndex;
	while (__secondChild < (__len - 1) / 2)
	  {
	    __secondChild = 2 * (__secondChild + 1);
	    if (__comp(*(__first + __secondChild),
		       *(__first + (__secondChild - 1))))
	      __secondChild--;
	    *(__first + __holeIndex) = ranges::iter_move(__first + __secondChild);
	    __holeIndex = __secondChild;
	  }
	if ((__len & 1) == 0 && __secondChild == (__len - 2) / 2)
	  {
	    __secondChild = 2 * (__secondChild + 1);
	    *(__first + __holeIndex) = ranges::iter_move(__first + (__secondChild - 1));
	    __holeIndex = __secondChild - 1;
	  }
	__detail::__push_heap(__first, __holeIndex, __topIndex,
			      std::move(__value), __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __pop_heap(_Iter __first, _Iter __last, _Iter __result, _Comp __comp)
      {
	iter_value_t<_Iter> __value = ranges::iter_move(__result);
	*__result = ranges::iter_move(__first);
	__detail::__adjust_heap(__first, 0, __last - __first,
				std::move(__value), __comp);
      }
  } // namespace __detail

  struct __pop_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    if (__last - __first > 1)
	      {
		auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
		__detail::__pop_heap(__first, __last - 1, __last - 1, __comp_proj);
	      }
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __pop_heap_fn pop_heap{};

  struct __make_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    const auto __len = __last - __first;
	    if (__len < 2)
	      return __last;

	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	    auto __parent = (__len - 2) / 2;
	    while (true)
	      {
		iter_value_t<_Iter> __value = ranges::iter_move(__first + __parent);
		__detail::__adjust_heap(__first, __parent, __len,
					std::move(__value),
					__comp_proj);
		if (__parent == 0)
		  break;
		__parent--;
	      }
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __make_heap_fn make_heap{};

  struct __sort_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	    _Iter __ret = __last;
	    while (__last - __first > 1)
	      {
		--__last;
		__detail::__pop_heap(__first, __last, __last, __comp_proj);
	      }
	    return __ret;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __sort_heap_fn sort_heap{};

  struct __is_heap_until_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	iter_difference_t<_Iter> __n = ranges::distance(__first, __last);
	iter_difference_t<_Iter> __parent = 0, __child = 1;
	for (; __child < __n; ++__child)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *(__first + __parent)),
			    std::__invoke(__proj, *(__first + __child))))
	    return __first + __child;
	  else if ((__child & 1) == 0)
	    ++__parent;

	return __first + __n;
      }

    template<random_access_range _Range,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_heap_until_fn is_heap_until{};

  struct __is_heap_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (__last
		== ranges::is_heap_until(__first, __last,
					 std::move(__comp),
					 std::move(__proj)));
      }

    template<random_access_range _Range,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_heap_fn is_heap{};

  namespace __detail
  {
    template<typename _Iter, typename _Comp>
      constexpr void
      __move_median_to_first(_Iter __result, _Iter __a, _Iter __b, _Iter __c,
			     _Comp __comp)
      {
	if (__comp(*__a, *__b))
	  {
	    if (__comp(*__b, *__c))
	      ranges::iter_swap(__result, __b);
	    else if (__comp(*__a, *__c))
	      ranges::iter_swap(__result, __c);
	    else
	      ranges::iter_swap(__result, __a);
	  }
	else if (__comp(*__a, *__c))
	  ranges::iter_swap(__result, __a);
	else if (__comp(*__b, *__c))
	  ranges::iter_swap(__result, __c);
	else
	  ranges::iter_swap(__result, __b);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __unguarded_linear_insert(_Iter __last, _Comp __comp)
      {
	iter_value_t<_Iter> __val = ranges::iter_move(__last);
	_Iter __next = __last;
	--__next;
	while (__comp(__val, *__next))
	  {
	    *__last = ranges::iter_move(__next);
	    __last = __next;
	    --__next;
	  }
	*__last = std::move(__val);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __insertion_sort(_Iter __first, _Iter __last, _Comp __comp)
      {
	if (__first == __last)
	  return;

	for (_Iter __i = __first + 1; __i != __last; ++__i)
	  {
	    if (__comp(*__i, *__first))
	      {
		iter_value_t<_Iter> __val = ranges::iter_move(__i);
		ranges::move_backward(__first, __i, __i + 1);
		*__first = std::move(__val);
	      }
	    else
	      __detail::__unguarded_linear_insert(__i, __comp);
	  }
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __unguarded_insertion_sort(_Iter __first, _Iter __last, _Comp __comp)
      {
	for (_Iter __i = __first; __i != __last; ++__i)
	  __detail::__unguarded_linear_insert(__i, __comp);
      }

    inline constexpr int __sort_threshold = 16;

    template<typename _Iter, typename _Comp>
      constexpr void
      __final_insertion_sort(_Iter __first, _Iter __last, _Comp __comp)
      {
	if (__last - __first > __sort_threshold)
	  {
	    __detail::__insertion_sort(__first, __first + __sort_threshold, __comp);
	    __detail::__unguarded_insertion_sort(__first + __sort_threshold, __last,
						 __comp);
	  }
	else
	  __detail::__insertion_sort(__first, __last, __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr _Iter
      __unguarded_partition(_Iter __first, _Iter __last, _Iter __pivot, _Comp __comp)
      {
	while (true)
	  {
	    while (__comp(*__first, *__pivot))
	      ++__first;
	    --__last;
	    while (__comp(*__pivot, *__last))
	      --__last;
	    if (!(__first < __last))
	      return __first;
	    ranges::iter_swap(__first, __last);
	    ++__first;
	  }
      }

    template<typename _Iter, typename _Comp>
      constexpr _Iter
      __unguarded_partition_pivot(_Iter __first, _Iter __last, _Comp __comp)
      {
	_Iter __mid = __first + (__last - __first) / 2;
	__detail::__move_median_to_first(__first, __first + 1, __mid, __last - 1, __comp);
	return __detail::__unguarded_partition(__first + 1, __last, __first, __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __heap_select(_Iter __first, _Iter __middle, _Iter __last, _Comp __comp)
      {
	ranges::make_heap(__first, __middle, __comp);
	for (_Iter __i = __middle; __i < __last; ++__i)
	  if (__comp(*__i, *__first))
	    __detail::__pop_heap(__first, __middle, __i, __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __partial_sort(_Iter __first, _Iter __middle, _Iter __last, _Comp __comp)
      {
	__detail::__heap_select(__first, __middle, __last, __comp);
	ranges::sort_heap(__first, __middle, __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __introsort_loop(_Iter __first, _Iter __last, unsigned __depth_limit, _Comp __comp)
      {
	while (__last - __first > __sort_threshold)
	  {
	    if (__depth_limit == 0)
	      {
		__detail::__partial_sort(__first, __last, __last, __comp);
		return;
	      }
	    --__depth_limit;
	    _Iter __cut = __detail::__unguarded_partition_pivot(__first, __last, __comp);
	    __detail::__introsort_loop(__cut, __last, __depth_limit, __comp);
	    __last = __cut;
	  }
      }
  } // namespace __detail

  struct __sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    if (__first != __last)
	      {
		auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
		auto __n = __detail::__to_unsigned_like(__last - __first);
		unsigned __depth_limit = (std::__bit_width(__n) - 1) * 2;
		__detail::__introsort_loop(__first, __last, __depth_limit, __comp_proj);
		__detail::__final_insertion_sort(__first, __last, __comp_proj);
	      }
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __sort_fn sort{};

  namespace __detail
  {
    // This is a helper function for the __merge_sort_loop routines.
    template<typename _Iter, typename _Out, typename _Comp>
      _Out
      __move_merge(_Iter __first1, _Iter __last1,
		   _Iter __first2, _Iter __last2,
		   _Out __result, _Comp __comp)
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (__comp(*__first2, *__first1))
	      {
		*__result = ranges::iter_move(__first2);
		++__first2;
	      }
	    else
	      {
		*__result = ranges::iter_move(__first1);
		++__first1;
	      }
	    ++__result;
	  }
	return ranges::move(__first2, __last2,
			    ranges::move(__first1, __last1, __result).out).out;
      }

    template<typename _Iter, typename _Out, typename _Distance, typename _Comp>
      void
      __merge_sort_loop(_Iter __first, _Iter __last, _Out __result,
			_Distance __step_size, _Comp __comp)
      {
	const _Distance __two_step = 2 * __step_size;

	while (__last - __first >= __two_step)
	  {
	    __result = __detail::__move_merge(__first, __first + __step_size,
					      __first + __step_size,
					      __first + __two_step,
					      __result, __comp);
	    __first += __two_step;
	  }
	__step_size = ranges::min(_Distance(__last - __first), __step_size);

	__detail::__move_merge(__first, __first + __step_size,
			       __first + __step_size, __last, __result, __comp);
      }

    template<typename _Iter, typename _Distance, typename _Compare>
      constexpr void
      __chunk_insertion_sort(_Iter __first, _Iter __last,
			     _Distance __chunk_size, _Compare __comp)
      {
	while (__last - __first >= __chunk_size)
	  {
	    __detail::__insertion_sort(__first, __first + __chunk_size, __comp);
	    __first += __chunk_size;
	  }
	__detail::__insertion_sort(__first, __last, __comp);
      }

    template<typename _Iter, typename _Pointer, typename _Comp>
      void
      __merge_sort_with_buffer(_Iter __first, _Iter __last,
			       _Pointer __buffer, _Comp __comp)
      {
	using _Distance = iter_difference_t<_Iter>;

	const _Distance __len = __last - __first;
	const _Pointer __buffer_last = __buffer + ptrdiff_t(__len);

	constexpr int __chunk_size = 7;
	_Distance __step_size = __chunk_size;
	__detail::__chunk_insertion_sort(__first, __last, __step_size, __comp);

	while (__step_size < __len)
	  {
	    __detail::__merge_sort_loop(__first, __last, __buffer,
					__step_size, __comp);
	    __step_size *= 2;
	    __detail::__merge_sort_loop(__buffer, __buffer_last, __first,
					ptrdiff_t(__step_size), __comp);
	    __step_size *= 2;
	  }
      }

    template<typename _Iter, typename _Pointer, typename _Comp>
      void
      __merge_adaptive(_Iter __first, _Iter __middle, _Iter __last,
		       iter_difference_t<_Iter> __len1,
		       iter_difference_t<_Iter> __len2,
		       _Pointer __buffer, _Comp __comp); // defined near inplace_merge

    template<typename _Iter, typename _Distance, typename _Pointer, typename _Comp>
      void
      __merge_adaptive_resize(_Iter __first, _Iter __middle, _Iter __last,
			      _Distance __len1, _Distance __len2,
			      _Pointer __buffer, _Distance __buffer_size,
			      _Comp __comp); // defined near inplace_merge

    template<typename _Iter, typename _Distance, typename _Comp>
      constexpr void
      __merge_without_buffer(_Iter __first, _Iter __middle, _Iter __last,
			     _Distance __len1, _Distance __len2,
			     _Comp __comp); // defined near inplace_merge

    template<typename _Iter, typename _Pointer, typename _Comp>
      void
      __stable_sort_adaptive(_Iter __first, _Iter __middle, _Iter __last,
			     _Pointer __buffer, _Comp __comp)
      {
	__detail::__merge_sort_with_buffer(__first, __middle, __buffer, __comp);
	__detail::__merge_sort_with_buffer(__middle, __last, __buffer, __comp);

	__detail::__merge_adaptive(__first, __middle, __last,
				   __middle - __first, __last - __middle,
				   __buffer, __comp);
      }

    template<typename _Iter, typename _Pointer, typename _Distance, typename _Comp>
      void
      __stable_sort_adaptive_resize(_Iter __first, _Iter __last,
				    _Pointer __buffer, _Distance __buffer_size,
				    _Comp __comp)
      {
	const _Distance __len = (__last - __first + 1) / 2;
	const _Iter __middle = __first + __len;
	if (__len > __buffer_size)
	  {
	    __detail::__stable_sort_adaptive_resize(__first, __middle, __buffer,
						    __buffer_size, __comp);
	    __detail::__stable_sort_adaptive_resize(__middle, __last, __buffer,
						    __buffer_size, __comp);
	    __detail::__merge_adaptive_resize(__first, __middle, __last,
					      _Distance(__middle - __first),
					      _Distance(__last - __middle),
					      __buffer, __buffer_size,
					      __comp);
	  }
	else
	  __detail::__stable_sort_adaptive(__first, __middle, __last,
					   __buffer, __comp);
      }

    template<typename _Iter, typename _Comp>
      constexpr void
      __inplace_stable_sort(_Iter __first, _Iter __last, _Comp __comp)
      {
	if (__last - __first < 15)
	  {
	    __detail::__insertion_sort(__first, __last, __comp);
	    return;
	  }
	_Iter __middle = __first + (__last - __first) / 2;
	__detail::__inplace_stable_sort(__first, __middle, __comp);
	__detail::__inplace_stable_sort(__middle, __last, __comp);
	__detail::__merge_without_buffer(__first, __middle, __last,
					 __middle - __first,
					 __last - __middle,
					 __comp);
      }
  } // namespace __detail

  struct __stable_sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      _GLIBCXX26_CONSTEXPR
      _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    using _DistanceType = iter_difference_t<_Iter>;

	    if (__first == __last)
	      return __last;

	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);

#if _GLIBCXX_HOSTED
# if __glibcxx_constexpr_algorithms >= 202306L // >= C++26
	    if consteval {
	      __detail::__inplace_stable_sort(__first, __last, __comp_proj);
	      return __last;
	    }
# endif

	    typedef _Temporary_buffer<_Iter, iter_value_t<_Iter>> _TmpBuf;
	    // __stable_sort_adaptive sorts the range in two halves,
	    // so the buffer only needs to fit half the range at once.
	    _TmpBuf __buf(__first, ptrdiff_t((__last - __first + 1) / 2));

	    if (__buf._M_requested_size() == __buf.size()) [[likely]]
	      __detail::__stable_sort_adaptive(__first,
					       __first + _DistanceType(__buf.size()),
					       __last, __buf.begin(), __comp_proj);
	    else if (__buf.begin()) [[unlikely]]
	      __detail::__inplace_stable_sort(__first, __last, __comp_proj);
	    else
	      __detail::__stable_sort_adaptive_resize(__first, __last, __buf.begin(),
						      _DistanceType(__buf.size()),
						      __comp_proj);
#else
	    __detail::__inplace_stable_sort(__first, __last, __comp_proj);
#endif
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      _GLIBCXX26_CONSTEXPR
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __stable_sort_fn stable_sort{};

  struct __partial_sort_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __middle)
	  return ranges::next(__first, __last);

	ranges::make_heap(__first, __middle, __comp, __proj);
	auto __i = __middle;
	for (; __i != __last; ++__i)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__i),
			    std::__invoke(__proj, *__first)))
	    {
	      ranges::pop_heap(__first, __middle, __comp, __proj);
	      ranges::iter_swap(__middle-1, __i);
	      ranges::push_heap(__first, __middle, __comp, __proj);
	    }
	ranges::sort_heap(__first, __middle, __comp, __proj);

	return __i;
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __partial_sort_fn partial_sort{};

  template<typename _Iter, typename _Out>
    using partial_sort_copy_result = in_out_result<_Iter, _Out>;

  struct __partial_sort_copy_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     random_access_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_copyable<_Iter1, _Iter2>
	&& sortable<_Iter2, _Comp, _Proj2>
	&& indirect_strict_weak_order<_Comp,
				      projected<_Iter1, _Proj1>,
				      projected<_Iter2, _Proj2>>
      constexpr partial_sort_copy_result<_Iter1, _Iter2>
      operator()(_Iter1 __first, _Sent1 __last,
		 _Iter2 __result_first, _Sent2 __result_last,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if (__result_first == __result_last)
	  {
	    // TODO: Eliminating the variable __lasti triggers an ICE.
	    auto __lasti = ranges::next(std::move(__first),
					std::move(__last));
	    return {std::move(__lasti), std::move(__result_first)};
	  }

	auto __result_real_last = __result_first;
	while (__first != __last && __result_real_last != __result_last)
	  {
	    *__result_real_last = *__first;
	    ++__result_real_last;
	    ++__first;
	  }

	ranges::make_heap(__result_first, __result_real_last, __comp, __proj2);
	for (; __first != __last; ++__first)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first),
			    std::__invoke(__proj2, *__result_first)))
	    {
	      ranges::pop_heap(__result_first, __result_real_last,
			       __comp, __proj2);
	      *(__result_real_last-1) = *__first;
	      ranges::push_heap(__result_first, __result_real_last,
				__comp, __proj2);
	    }
	ranges::sort_heap(__result_first, __result_real_last, __comp, __proj2);

	return {std::move(__first), std::move(__result_real_last)};
      }

    template<input_range _Range1, random_access_range _Range2,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_copyable<iterator_t<_Range1>, iterator_t<_Range2>>
	&& sortable<iterator_t<_Range2>, _Comp, _Proj2>
	&& indirect_strict_weak_order<_Comp,
				      projected<iterator_t<_Range1>, _Proj1>,
				      projected<iterator_t<_Range2>, _Proj2>>
      constexpr partial_sort_copy_result<borrowed_iterator_t<_Range1>,
					 borrowed_iterator_t<_Range2>>
      operator()(_Range1&& __r, _Range2&& __out, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       ranges::begin(__out), ranges::end(__out),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __partial_sort_copy_fn partial_sort_copy{};

  struct __is_sorted_until_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __next = __first;
	for (++__next; __next != __last; __first = __next, (void)++__next)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__next),
			    std::__invoke(__proj, *__first)))
	    return __next;
	return __next;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_sorted_until_fn is_sorted_until{};

  struct __is_sorted_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return true;

	auto __next = __first;
	for (++__next; __next != __last; __first = __next, (void)++__next)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj, *__next),
			    std::__invoke(__proj, *__first)))
	    return false;
	return true;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __is_sorted_fn is_sorted{};

  namespace __detail
  {
    template<typename _Iter, typename _Comp>
      constexpr void
      __introselect(_Iter __first, _Iter __nth, _Iter __last,
		    iter_difference_t<_Iter> __depth_limit, _Comp __comp)
      {
	while (__last - __first > 3)
	  {
	    if (__depth_limit == 0)
	      {
		__detail::__heap_select(__first, __nth + 1, __last, __comp);
		// Place the nth largest element in its final position.
		ranges::iter_swap(__first, __nth);
		return;
	      }
	    --__depth_limit;
	    _Iter __cut = __detail::__unguarded_partition_pivot(__first, __last, __comp);
	    if (__cut <= __nth)
	      __first = __cut;
	    else
	      __last = __cut;
	  }
	__detail::__insertion_sort(__first, __last, __comp);
      }
  } // namespace __detail

  struct __nth_element_fn
  {
    template<random_access_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr _Iter
      operator()(_Iter __first, _Iter __nth, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, __nth, ranges::next(__first, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    if (__first == __last || __nth == __last)
	      return __last;

	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	    auto __n = __detail::__to_unsigned_like(__last - __first);
	    __detail::__introselect(__first, __nth, __last,
				    std::__bit_width(__n) * 2,
				    __comp_proj);
	    return __last;
	  }
      }

    template<random_access_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __nth,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__nth),
		       ranges::end(__r), std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __nth_element_fn nth_element{};

  struct __lower_bound_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp, std::__invoke(__proj, *__middle), __value))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else
	      __len = __half;
	  }
	return __first;
      }

    template<forward_range _Range,
	     typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __lower_bound_fn lower_bound{};

  struct __upper_bound_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp, __value, std::__invoke(__proj, *__middle)))
	      __len = __half;
	    else
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	  }
	return __first;
      }

    template<forward_range _Range,
	     typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __upper_bound_fn upper_bound{};

  struct __equal_range_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__middle),
			      __value))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else if (std::__invoke(__comp,
				   __value,
				   std::__invoke(__proj, *__middle)))
	      __len = __half;
	    else
	      {
		auto __left
		  = ranges::lower_bound(__first, __middle,
					__value, __comp, __proj);
		ranges::advance(__first, __len);
		auto __right
		  = ranges::upper_bound(++__middle, __first,
					__value, __comp, __proj);
		return {__left, __right};
	      }
	  }
	return {__first, __first};
      }

    template<forward_range _Range,
	     typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, const _Tp& __value,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __equal_range_fn equal_range{};

  struct __binary_search_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj),
	     indirect_strict_weak_order<const _Tp*, projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 const _Tp& __value, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __i = ranges::lower_bound(__first, __last, __value, __comp, __proj);
	if (__i == __last)
	  return false;
	return !(bool)std::__invoke(__comp, __value,
				    std::__invoke(__proj, *__i));
      }

    template<forward_range _Range,
	     typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj),
	     indirect_strict_weak_order<const _Tp*,
					projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range&& __r, const _Tp& __value, _Comp __comp = {},
		 _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       __value, std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __binary_search_fn binary_search{};

  struct __is_partitioned_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr bool
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	__first = ranges::find_if_not(std::move(__first), __last,
				      __pred, __proj);
	if (__first == __last)
	  return true;
	++__first;
	return ranges::none_of(std::move(__first), std::move(__last),
			       std::move(__pred), std::move(__proj));
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr bool
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __is_partitioned_fn is_partitioned{};

  struct __partition_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	if constexpr (bidirectional_iterator<_Iter>)
	  {
	    auto __lasti = ranges::next(__first, __last);
	    auto __tail = __lasti;
	    for (;;)
	      {
		for (;;)
		  if (__first == __tail)
		    return {std::move(__first), std::move(__lasti)};
		  else if (std::__invoke(__pred,
					 std::__invoke(__proj, *__first)))
		    ++__first;
		  else
		    break;
		--__tail;
		for (;;)
		  if (__first == __tail)
		    return {std::move(__first), std::move(__lasti)};
		  else if (!(bool)std::__invoke(__pred,
						std::__invoke(__proj, *__tail)))
		    --__tail;
		  else
		    break;
		ranges::iter_swap(__first, __tail);
		++__first;
	      }
	  }
	else
	  {
	    if (__first == __last)
	      return {__first, __first};

	    while (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	      if (++__first == __last)
		return {__first, __first};

	    auto __next = __first;
	    while (++__next != __last)
	      if (std::__invoke(__pred, std::__invoke(__proj, *__next)))
		{
		  ranges::iter_swap(__first, __next);
		  ++__first;
		}

	    return {std::move(__first), std::move(__next)};
	  }
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_fn partition{};

#if _GLIBCXX_HOSTED
  namespace __detail
  {
    // Like find_if_not(), but uses and updates a count of the
    // remaining range length instead of comparing against an end
    // iterator.
    template<typename _Iter, typename _Pred, typename _Distance>
      constexpr _Iter
      __find_if_not_n(_Iter __first, _Distance& __len, _Pred __pred)
      {
	for (; __len; --__len,  (void) ++__first)
	  if (!__pred(*__first))
	    break;
	return __first;
      }

    template<typename _Iter, typename _Sent, typename _Pointer,
	     typename _Pred, typename _Distance>
      constexpr subrange<_Iter>
      __stable_partition_adaptive(_Iter __first, _Sent __last,
				  _Pred __pred, _Distance __len,
				  _Pointer __buffer,
				  _Distance __buffer_size)
      {
	if (__len == 1)
	  return {__first, ranges::next(__first, 1)};

	if (__len <= __buffer_size)
	  {
	    _Iter __result1 = __first;
	    _Pointer __result2 = __buffer;

	    // The precondition guarantees that !__pred(__first), so
	    // move that element to the buffer before starting the loop.
	    // This ensures that we only call __pred once per element.
	    *__result2 = ranges::iter_move(__first);
	    ++__result2;
	    ++__first;
	    for (; __first != __last; ++__first)
	      if (__pred(*__first))
		{
		  *__result1 = ranges::iter_move(__first);
		  ++__result1;
		}
	      else
		{
		  *__result2 = ranges::iter_move(__first);
		  ++__result2;
		}

	    ranges::move(__buffer, __result2, __result1);
	    return {__result1, __first};
	  }

	_Iter __middle = __first;
	ranges::advance(__middle, __len / 2);
	_Iter __left_split
	  = __detail::__stable_partition_adaptive(__first, __middle, __pred,
						  __len / 2, __buffer,
						  __buffer_size).begin();

	// Advance past true-predicate values to satisfy this
	// function's preconditions.
	_Distance __right_len = __len - __len / 2;
	_Iter __right_split = __detail::__find_if_not_n(__middle, __right_len, __pred);

	if (__right_len)
	  __right_split
	    = __detail::__stable_partition_adaptive(__right_split, __last, __pred,
						    __right_len, __buffer, __buffer_size).begin();

	return ranges::rotate(__left_split, __middle, __right_split);
      }
  } // namespace __detail

  struct __stable_partition_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires permutable<_Iter>
      _GLIBCXX26_CONSTEXPR
      subrange<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	__first = ranges::find_if_not(__first, __last, __pred, __proj);

	if (__first == __last)
	  return {__first, __first};

	using _DistanceType = iter_difference_t<_Iter>;
	const _DistanceType __len = ranges::distance(__first, __last);

	auto __pred_proj = __detail::__make_pred_proj(__pred, __proj);

#if __glibcxx_constexpr_algorithms >= 202306L // >= C++26
	if consteval {
	  // Simulate a _Temporary_buffer of length 1:
	  iter_value_t<_Iter> __buf = ranges::iter_move(__first);
	  *__first = std::move(__buf);
	  return __detail::__stable_partition_adaptive(__first, __last,
						       __pred_proj,
						       __len, &__buf,
						       _DistanceType(1));
	}
#endif

	_Temporary_buffer<_Iter, iter_value_t<_Iter>> __buf(__first, ptrdiff_t(__len));
	return __detail::__stable_partition_adaptive(__first, __last,
						     __pred_proj,
						     __len, __buf.begin(),
						     _DistanceType(__buf.size()));
      }

    template<bidirectional_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires permutable<iterator_t<_Range>>
      _GLIBCXX26_CONSTEXPR
      borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __stable_partition_fn stable_partition{};
#endif

  template<typename _Iter, typename _Out1, typename _Out2>
    struct in_out_out_result
    {
      [[no_unique_address]] _Iter  in;
      [[no_unique_address]] _Out1 out1;
      [[no_unique_address]] _Out2 out2;

      template<typename _IIter, typename _OOut1, typename _OOut2>
	requires convertible_to<const _Iter&, _IIter>
	  && convertible_to<const _Out1&, _OOut1>
	  && convertible_to<const _Out2&, _OOut2>
	constexpr
	operator in_out_out_result<_IIter, _OOut1, _OOut2>() const &
	{ return {in, out1, out2}; }

      template<typename _IIter, typename _OOut1, typename _OOut2>
	requires convertible_to<_Iter, _IIter>
	  && convertible_to<_Out1, _OOut1>
	  && convertible_to<_Out2, _OOut2>
	constexpr
	operator in_out_out_result<_IIter, _OOut1, _OOut2>() &&
	{ return {std::move(in), std::move(out1), std::move(out2)}; }
    };

  template<typename _Iter, typename _Out1, typename _Out2>
    using partition_copy_result = in_out_out_result<_Iter, _Out1, _Out2>;

  struct __partition_copy_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     weakly_incrementable _Out1, weakly_incrementable _Out2,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      requires indirectly_copyable<_Iter, _Out1>
	&& indirectly_copyable<_Iter, _Out2>
      constexpr partition_copy_result<_Iter, _Out1, _Out2>
      operator()(_Iter __first, _Sent __last,
		 _Out1 __out_true, _Out2 __out_false,
		 _Pred __pred, _Proj __proj = {}) const
      {
	for (; __first != __last; ++__first)
	  if (std::__invoke(__pred, std::__invoke(__proj, *__first)))
	    {
	      *__out_true = *__first;
	      ++__out_true;
	    }
	  else
	    {
	      *__out_false = *__first;
	      ++__out_false;
	    }

	return {std::move(__first),
		std::move(__out_true), std::move(__out_false)};
      }

    template<input_range _Range, weakly_incrementable _Out1,
	     weakly_incrementable _Out2,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      requires indirectly_copyable<iterator_t<_Range>, _Out1>
	&& indirectly_copyable<iterator_t<_Range>, _Out2>
      constexpr partition_copy_result<borrowed_iterator_t<_Range>, _Out1, _Out2>
      operator()(_Range&& __r, _Out1 __out_true, _Out2 __out_false,
		 _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__out_true), std::move(__out_false),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_copy_fn partition_copy{};

  struct __partition_point_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Pred __pred, _Proj __proj = {}) const
      {
	auto __len = ranges::distance(__first, __last);

	while (__len > 0)
	  {
	    auto __half = __len / 2;
	    auto __middle = __first;
	    ranges::advance(__middle, __half);
	    if (std::__invoke(__pred, std::__invoke(__proj, *__middle)))
	      {
		__first = __middle;
		++__first;
		__len = __len - __half - 1;
	      }
	    else
	      __len = __half;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>>
	       _Pred>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__pred), std::move(__proj));
      }
  };

  inline constexpr __partition_point_fn partition_point{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using merge_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __merge_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr merge_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj2, *__first2),
			      std::__invoke(__proj1, *__first1)))
	      {
		*__result = *__first2;
		++__first2;
	      }
	    else
	      {
		*__result = *__first1;
		++__first1;
	      }
	    ++__result;
	  }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return { std::move(__copy1.in), std::move(__copy2.in),
		 std::move(__copy2.out) };
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr merge_result<borrowed_iterator_t<_Range1>,
			     borrowed_iterator_t<_Range2>,
			     _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __merge_fn merge{};

  namespace __detail
  {
    template<typename _Iter1, typename _Iter2, typename _Out, typename _Comp>
      void
      __move_merge_adaptive(_Iter1 __first1, _Iter1 __last1,
			    _Iter2 __first2, _Iter2 __last2,
			    _Out __result, _Comp __comp)
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (__comp(*__first2, *__first1))
	      {
		*__result = ranges::iter_move(__first2);
		++__first2;
	      }
	    else
	      {
		*__result = ranges::iter_move(__first1);
		++__first1;
	      }
	    ++__result;
	  }
	if (__first1 != __last1)
	  ranges::move(__first1, __last1, __result);
      }

    template<typename _Iter1, typename _Iter2, typename _Iter3, typename _Comp>
      void
      __move_merge_adaptive_backward(_Iter1 __first1, _Iter1 __last1,
				     _Iter2 __first2, _Iter2 __last2,
				     _Iter3 __result, _Comp __comp)
      {
	if (__first1 == __last1)
	  {
	    ranges::move_backward(__first2, __last2, __result);
	    return;
	  }
	else if (__first2 == __last2)
	  return;

	--__last1;
	--__last2;
	while (true)
	  {
	    if (__comp(*__last2, *__last1))
	      {
		*--__result = ranges::iter_move(__last1);
		if (__first1 == __last1)
		  {
		    ranges::move_backward(__first2, ++__last2, __result);
		    return;
		  }
		--__last1;
	      }
	    else
	      {
		*--__result = ranges::iter_move(__last2);
		if (__first2 == __last2)
		  return;
		--__last2;
	      }
	  }
      }

    template<typename _Iter1, typename _Iter2>
      _Iter1
      __rotate_adaptive(_Iter1 __first, _Iter1 __middle, _Iter1 __last,
			iter_difference_t<_Iter1> __len1,
			iter_difference_t<_Iter1> __len2,
			_Iter2 __buffer,
			iter_difference_t<_Iter1> __buffer_size)
      {
	_Iter2 __buffer_end;
	if (__len1 > __len2 && __len2 <= __buffer_size)
	  {
	    if (__len2)
	      {
		__buffer_end = ranges::move(__middle, __last, __buffer).out;
		ranges::move_backward(__first, __middle, __last);
		return ranges::move(__buffer, __buffer_end, __first).out;
	      }
	    else
	      return __first;
	  }
	else if (__len1 <= __buffer_size)
	  {
	    if (__len1)
	      {
		__buffer_end = ranges::move(__first, __middle, __buffer).out;
		ranges::move(__middle, __last, __first);
		return ranges::move_backward(__buffer, __buffer_end, __last).out;
	      }
	    else
	      return __last;
	  }
	else
	  return ranges::rotate(__first, __middle, __last).begin();
      }

    template<typename _Iter, typename _Pointer, typename _Comp>
      void
      __merge_adaptive(_Iter __first, _Iter __middle, _Iter __last,
		       iter_difference_t<_Iter> __len1,
		       iter_difference_t<_Iter> __len2,
		       _Pointer __buffer, _Comp __comp)
      {
	if (__len1 <= __len2)
	  {
	    _Pointer __buffer_end = ranges::move(__first, __middle, __buffer).out;
	    __detail::__move_merge_adaptive(__buffer, __buffer_end, __middle, __last,
					    __first, __comp);
	  }
	else
	  {
	    _Pointer __buffer_end = ranges::move(__middle, __last, __buffer).out;
	    __detail::__move_merge_adaptive_backward(__first, __middle, __buffer,
						     __buffer_end, __last, __comp);
	  }
      }

    template<typename _Iter, typename _Distance, typename _Pointer, typename _Comp>
      void
      __merge_adaptive_resize(_Iter __first, _Iter __middle, _Iter __last,
			      _Distance __len1, _Distance __len2,
			      _Pointer __buffer, _Distance __buffer_size,
			      _Comp __comp)
      {
	if (__len1 <= __buffer_size || __len2 <= __buffer_size)
	  __detail::__merge_adaptive(__first, __middle, __last,
				     __len1, __len2, __buffer, __comp);
	else
	  {
	    _Iter __first_cut = __first;
	    _Iter __second_cut = __middle;
	    _Distance __len11 = 0;
	    _Distance __len22 = 0;
	    if (__len1 > __len2)
	      {
		__len11 = __len1 / 2;
		ranges::advance(__first_cut, __len11);
		__second_cut = ranges::lower_bound(__middle, __last, *__first_cut,
						   __comp);
		__len22 = ranges::distance(__middle, __second_cut);
	      }
	    else
	      {
		__len22 = __len2 / 2;
		ranges::advance(__second_cut, __len22);
		__first_cut = ranges::upper_bound(__first, __middle, *__second_cut,
						  __comp);
		__len11 = ranges::distance(__first, __first_cut);
	      }

	    _Iter __new_middle
	      = __detail::__rotate_adaptive(__first_cut, __middle, __second_cut,
					    _Distance(__len1 - __len11), __len22,
					    __buffer, __buffer_size);
	    __detail::__merge_adaptive_resize(__first, __first_cut, __new_middle,
					      __len11, __len22,
					      __buffer, __buffer_size, __comp);
	    __detail::__merge_adaptive_resize(__new_middle, __second_cut, __last,
					      _Distance(__len1 - __len11),
					      _Distance(__len2 - __len22),
					      __buffer, __buffer_size, __comp);
	  }
      }

    template<typename _Iter, typename _Distance, typename _Comp>
      constexpr void
      __merge_without_buffer(_Iter __first, _Iter __middle, _Iter __last,
			     _Distance __len1, _Distance __len2, _Comp __comp)
      {
	if (__len1 == 0 || __len2 == 0)
	  return;

	if (__len1 + __len2 == 2)
	  {
	    if (__comp(*__middle, *__first))
	      ranges::iter_swap(__first, __middle);
	    return;
	  }

	_Iter __first_cut = __first;
	_Iter __second_cut = __middle;
	_Distance __len11 = 0;
	_Distance __len22 = 0;
	if (__len1 > __len2)
	  {
	    __len11 = __len1 / 2;
	    ranges::advance(__first_cut, __len11);
	    __second_cut = ranges::lower_bound(__middle, __last, *__first_cut, __comp);
	    __len22 = ranges::distance(__middle, __second_cut);
	  }
	else
	  {
	    __len22 = __len2 / 2;
	    ranges::advance(__second_cut, __len22);
	    __first_cut = ranges::upper_bound(__first, __middle, *__second_cut, __comp);
	    __len11 = ranges::distance(__first, __first_cut);
	  }

	_Iter __new_middle = ranges::rotate(__first_cut, __middle, __second_cut).begin();
	__detail::__merge_without_buffer(__first, __first_cut, __new_middle,
					 __len11, __len22, __comp);
	__detail::__merge_without_buffer(__new_middle, __second_cut, __last,
					 __len1 - __len11, __len2 - __len22, __comp);
      }
  } // namespace __detail

  struct __inplace_merge_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      _GLIBCXX26_CONSTEXPR
      _Iter
      operator()(_Iter __first, _Iter __middle, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if constexpr (!same_as<_Iter, _Sent>)
	  return (*this)(__first, __middle, ranges::next(__middle, __last),
			 std::move(__comp), std::move(__proj));
	else
	  {
	    using _DistanceType = iter_difference_t<_Iter>;

	    if (__first == __middle || __middle == __last)
	      return __last;

	    const _DistanceType __len1 = ranges::distance(__first, __middle);
	    const _DistanceType __len2 = ranges::distance(__middle, __last);

	    auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);

#if _GLIBCXX_HOSTED
# if __glibcxx_constexpr_algorithms >= 202306L // >= C++26
	    if consteval {
	      __detail::__merge_without_buffer(__first, __middle, __last,
					       __len1, __len2, __comp_proj);
	      return __last;
	    }
# endif
	    using _TmpBuf = _Temporary_buffer<_Iter, iter_value_t<_Iter>>;
	    // __merge_adaptive will use a buffer for the smaller of
	    // [first,middle) and [middle,last).
	    _TmpBuf __buf(__first, ptrdiff_t(ranges::min(__len1, __len2)));

	    if (__buf.size() == __buf._M_requested_size()) [[likely]]
	      __detail::__merge_adaptive
		(__first, __middle, __last, __len1, __len2, __buf.begin(), __comp_proj);
	    else if (__buf.begin() == 0) [[unlikely]]
	      __detail::__merge_without_buffer
		(__first, __middle, __last, __len1, __len2, __comp_proj);
	    else
	      __detail::__merge_adaptive_resize
		(__first, __middle, __last, __len1, __len2, __buf.begin(),
		 _DistanceType(__buf.size()), __comp_proj);
#else
	    __detail::__merge_without_buffer
	      (__first, __middle, __last, __len1, __len2, __comp_proj);
#endif
	    return __last;
	  }
      }

    template<bidirectional_range _Range,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      _GLIBCXX26_CONSTEXPR
      borrowed_iterator_t<_Range>
      operator()(_Range&& __r, iterator_t<_Range> __middle,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), std::move(__middle),
		       ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __inplace_merge_fn inplace_merge{};

  struct __includes_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<_Iter1, _Proj1>,
					projected<_Iter2, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj2, *__first2),
			    std::__invoke(__proj1, *__first1)))
	    return false;
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj1, *__first1),
				 std::__invoke(__proj2, *__first2)))
	    ++__first1;
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }

	return __first2 == __last2;
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range1>, _Proj1>,
					projected<iterator_t<_Range2>, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __includes_fn includes{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_union_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_union_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_union_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj1, *__first1),
			      std::__invoke(__proj2, *__first2)))
	      {
		*__result = *__first1;
		++__first1;
	      }
	    else if (std::__invoke(__comp,
				   std::__invoke(__proj2, *__first2),
				   std::__invoke(__proj1, *__first1)))
	      {
		*__result = *__first2;
		++__first2;
	      }
	    else
	      {
		*__result = *__first1;
		++__first1;
		++__first2;
	      }
	    ++__result;
	  }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return {std::move(__copy1.in), std::move(__copy2.in),
		std::move(__copy2.out)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_union_result<borrowed_iterator_t<_Range1>,
				 borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_union_fn set_union{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_intersection_result = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_intersection_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_intersection_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    ++__first1;
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    ++__first2;
	  else
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__first2;
	      ++__result;
	    }
	// TODO: Eliminating these variables triggers an ICE.
	auto __last1i = ranges::next(std::move(__first1), std::move(__last1));
	auto __last2i = ranges::next(std::move(__first2), std::move(__last2));
	return {std::move(__last1i), std::move(__last2i), std::move(__result)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_intersection_result<borrowed_iterator_t<_Range1>,
					borrowed_iterator_t<_Range2>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_intersection_fn set_intersection{};

  template<typename _Iter, typename _Out>
    using set_difference_result = in_out_result<_Iter, _Out>;

  struct __set_difference_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_difference_result<_Iter1, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__result;
	    }
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    ++__first2;
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }
	return ranges::copy(std::move(__first1), std::move(__last1),
			    std::move(__result));
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_difference_result<borrowed_iterator_t<_Range1>, _Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_difference_fn set_difference{};

  template<typename _Iter1, typename _Iter2, typename _Out>
    using set_symmetric_difference_result
      = in_in_out_result<_Iter1, _Iter2, _Out>;

  struct __set_symmetric_difference_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     weakly_incrementable _Out, typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<_Iter1, _Iter2, _Out, _Comp, _Proj1, _Proj2>
      constexpr set_symmetric_difference_result<_Iter1, _Iter2, _Out>
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Out __result, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	while (__first1 != __last1 && __first2 != __last2)
	  if (std::__invoke(__comp,
			    std::__invoke(__proj1, *__first1),
			    std::__invoke(__proj2, *__first2)))
	    {
	      *__result = *__first1;
	      ++__first1;
	      ++__result;
	    }
	  else if (std::__invoke(__comp,
				 std::__invoke(__proj2, *__first2),
				 std::__invoke(__proj1, *__first1)))
	    {
	      *__result = *__first2;
	      ++__first2;
	      ++__result;
	    }
	  else
	    {
	      ++__first1;
	      ++__first2;
	    }
	auto __copy1 = ranges::copy(std::move(__first1), std::move(__last1),
				    std::move(__result));
	auto __copy2 = ranges::copy(std::move(__first2), std::move(__last2),
				    std::move(__copy1.out));
	return {std::move(__copy1.in), std::move(__copy2.in),
		std::move(__copy2.out)};
      }

    template<input_range _Range1, input_range _Range2, weakly_incrementable _Out,
	     typename _Comp = ranges::less,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires mergeable<iterator_t<_Range1>, iterator_t<_Range2>, _Out,
			 _Comp, _Proj1, _Proj2>
      constexpr set_symmetric_difference_result<borrowed_iterator_t<_Range1>,
						borrowed_iterator_t<_Range2>,
						_Out>
      operator()(_Range1&& __r1, _Range2&& __r2, _Out __result,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__result), std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __set_symmetric_difference_fn set_symmetric_difference{};

  // min is defined in <bits/ranges_util.h>.

  struct __max_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr const _Tp&
      operator()(const _Tp& __a, const _Tp& __b,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (std::__invoke(__comp,
			  std::__invoke(__proj, __a),
			  std::__invoke(__proj, __b)))
	  return __b;
	else
	  return __a;
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      requires indirectly_copyable_storable<iterator_t<_Range>,
					    range_value_t<_Range>*>
      constexpr range_value_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __first = ranges::begin(__r);
	auto __last = ranges::end(__r);
	__glibcxx_assert(__first != __last);
	auto __result = *__first;
	while (++__first != __last)
	  {
	    auto&& __tmp = *__first;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, __result),
			      std::__invoke(__proj, __tmp)))
	      __result = std::forward<decltype(__tmp)>(__tmp);
	  }
	return __result;
      }

    template<copyable _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr _Tp
      operator()(initializer_list<_Tp> __r,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::subrange(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __max_fn max{};

  struct __clamp_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>> _Comp
	       = ranges::less>
      constexpr const _Tp&
      operator()(const _Tp& __val, const _Tp& __lo, const _Tp& __hi,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	__glibcxx_assert(!(std::__invoke(__comp,
					 std::__invoke(__proj, __hi),
					 std::__invoke(__proj, __lo))));
	auto&& __proj_val = std::__invoke(__proj, __val);
	if (std::__invoke(__comp,
			  std::forward<decltype(__proj_val)>(__proj_val),
			  std::__invoke(__proj, __lo)))
	  return __lo;
	else if (std::__invoke(__comp,
			       std::__invoke(__proj, __hi),
			       std::forward<decltype(__proj_val)>(__proj_val)))
	  return __hi;
	else
	  return __val;
      }
  };

  inline constexpr __clamp_fn clamp{};

  template<typename _Tp>
    struct min_max_result
    {
      [[no_unique_address]] _Tp min;
      [[no_unique_address]] _Tp max;

      template<typename _Tp2>
	requires convertible_to<const _Tp&, _Tp2>
	constexpr
	operator min_max_result<_Tp2>() const &
	{ return {min, max}; }

      template<typename _Tp2>
	requires convertible_to<_Tp, _Tp2>
	constexpr
	operator min_max_result<_Tp2>() &&
	{ return {std::move(min), std::move(max)}; }
    };

  template<typename _Tp>
    using minmax_result = min_max_result<_Tp>;

  struct __minmax_fn
  {
    template<typename _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_result<const _Tp&>
      operator()(const _Tp& __a, const _Tp& __b,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (std::__invoke(__comp,
			  std::__invoke(__proj, __b),
			  std::__invoke(__proj, __a)))
	  return {__b, __a};
	else
	  return {__a, __b};
      }

    template<input_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      requires indirectly_copyable_storable<iterator_t<_Range>, range_value_t<_Range>*>
      constexpr minmax_result<range_value_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __first = ranges::begin(__r);
	auto __last = ranges::end(__r);
	__glibcxx_assert(__first != __last);
	auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	minmax_result<range_value_t<_Range>> __result = {*__first, __result.min};
	if (++__first == __last)
	  return __result;
	else
	  {
	    // At this point __result.min == __result.max, so a single
	    // comparison with the next element suffices.
	    auto&& __val = *__first;
	    if (__comp_proj(__val, __result.min))
	      __result.min = std::forward<decltype(__val)>(__val);
	    else
	      __result.max = std::forward<decltype(__val)>(__val);
	  }
	while (++__first != __last)
	  {
	    // Now process two elements at a time so that we perform at most
	    // 1 + 3*(N-2)/2 comparisons in total (each of the (N-2)/2
	    // iterations of this loop performs three comparisons).
	    range_value_t<_Range> __val1 = *__first;
	    if (++__first == __last)
	      {
		// N is odd; in this final iteration, we perform at most two
		// comparisons, for a total of 1 + 3*(N-3)/2 + 2 comparisons,
		// which is not more than 3*N/2, as required.
		if (__comp_proj(__val1, __result.min))
		  __result.min = std::move(__val1);
		else if (!__comp_proj(__val1, __result.max))
		  __result.max = std::move(__val1);
		break;
	      }
	    auto&& __val2 = *__first;
	    if (!__comp_proj(__val2, __val1))
	      {
		if (__comp_proj(__val1, __result.min))
		  __result.min = std::move(__val1);
		if (!__comp_proj(__val2, __result.max))
		  __result.max = std::forward<decltype(__val2)>(__val2);
	      }
	    else
	      {
		if (__comp_proj(__val2, __result.min))
		  __result.min = std::forward<decltype(__val2)>(__val2);
		if (!__comp_proj(__val1, __result.max))
		  __result.max = std::move(__val1);
	      }
	  }
	return __result;
      }

    template<copyable _Tp, typename _Proj = identity,
	     indirect_strict_weak_order<projected<const _Tp*, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_result<_Tp>
      operator()(initializer_list<_Tp> __r,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::subrange(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __minmax_fn minmax{};

  struct __min_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __i = __first;
	while (++__i != __last)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__i),
			      std::__invoke(__proj, *__first)))
	      __first = __i;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __min_element_fn min_element{};

  struct __max_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr _Iter
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return __first;

	auto __i = __first;
	while (++__i != __last)
	  {
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__first),
			      std::__invoke(__proj, *__i)))
	      __first = __i;
	  }
	return __first;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr borrowed_iterator_t<_Range>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __max_element_fn max_element{};

  template<typename _Iter>
    using minmax_element_result = min_max_result<_Iter>;

  struct __minmax_element_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     indirect_strict_weak_order<projected<_Iter, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_element_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	auto __comp_proj = __detail::__make_comp_proj(__comp, __proj);
	minmax_element_result<_Iter> __result = {__first, __first};
	if (__first == __last || ++__first == __last)
	  return __result;
	else
	  {
	    // At this point __result.min == __result.max, so a single
	    // comparison with the next element suffices.
	    if (__comp_proj(*__first, *__result.min))
	      __result.min = __first;
	    else
	      __result.max = __first;
	  }
	while (++__first != __last)
	  {
	    // Now process two elements at a time so that we perform at most
	    // 1 + 3*(N-2)/2 comparisons in total (each of the (N-2)/2
	    // iterations of this loop performs three comparisons).
	    auto __prev = __first;
	    if (++__first == __last)
	      {
		// N is odd; in this final iteration, we perform at most two
		// comparisons, for a total of 1 + 3*(N-3)/2 + 2 comparisons,
		// which is not more than 3*N/2, as required.
		if (__comp_proj(*__prev, *__result.min))
		  __result.min = __prev;
		else if (!__comp_proj(*__prev, *__result.max))
		  __result.max = __prev;
		break;
	      }
	    if (!__comp_proj(*__first, *__prev))
	      {
		if (__comp_proj(*__prev, *__result.min))
		  __result.min = __prev;
		if (!__comp_proj(*__first, *__result.max))
		  __result.max = __first;
	      }
	    else
	      {
		if (__comp_proj(*__first, *__result.min))
		  __result.min = __first;
		if (!__comp_proj(*__prev, *__result.max))
		  __result.max = __prev;
	      }
	  }
	return __result;
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range>, _Proj>>
	       _Comp = ranges::less>
      constexpr minmax_element_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __minmax_element_fn minmax_element{};

  struct __lexicographical_compare_fn
  {
    template<input_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     input_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<_Iter1, _Proj1>,
					projected<_Iter2, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1,
		 _Iter2 __first2, _Sent2 __last2,
		 _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	if constexpr (__detail::__is_normal_iterator<_Iter1>
		      && same_as<_Iter1, _Sent1>)
	  return (*this)(__first1.base(), __last1.base(),
			 std::move(__first2), std::move(__last2),
			 std::move(__comp),
			 std::move(__proj1), std::move(__proj2));
	else if constexpr (__detail::__is_normal_iterator<_Iter2>
			   && same_as<_Iter2, _Sent2>)
	  return (*this)(std::move(__first1), std::move(__last1),
			 __first2.base(), __last2.base(),
			 std::move(__comp),
			 std::move(__proj1), std::move(__proj2));
	else
	  {
	    constexpr bool __sized_iters
	      = (sized_sentinel_for<_Sent1, _Iter1>
		 && sized_sentinel_for<_Sent2, _Iter2>);
	    if constexpr (__sized_iters)
	      {
		using _ValueType1 = iter_value_t<_Iter1>;
		using _ValueType2 = iter_value_t<_Iter2>;
		// This condition is consistent with the one in
		// __lexicographical_compare_aux in <bits/stl_algobase.h>.
		constexpr bool __use_memcmp
		  = (__is_memcmp_ordered_with<_ValueType1, _ValueType2>::__value
		     && __ptr_to_nonvolatile<_Iter1>
		     && __ptr_to_nonvolatile<_Iter2>
		     && (is_same_v<_Comp, ranges::less>
			 || is_same_v<_Comp, ranges::greater>)
		     && is_same_v<_Proj1, identity>
		     && is_same_v<_Proj2, identity>);
		if constexpr (__use_memcmp)
		  {
		    const auto __d1 = __last1 - __first1;
		    const auto __d2 = __last2 - __first2;

		    if (const auto __len = std::min(__d1, __d2))
		      {
			const auto __c
			  = std::__memcmp(__first1, __first2, __len);
			if constexpr (is_same_v<_Comp, ranges::less>)
			  {
			    if (__c < 0)
			      return true;
			    if (__c > 0)
			      return false;
			  }
			else if constexpr (is_same_v<_Comp, ranges::greater>)
			  {
			    if (__c > 0)
			      return true;
			    if (__c < 0)
			      return false;
			  }
		      }
		    return __d1 < __d2;
		  }
	      }

	    for (; __first1 != __last1 && __first2 != __last2;
		 ++__first1, (void) ++__first2)
	      {
		if (std::__invoke(__comp,
				  std::__invoke(__proj1, *__first1),
				  std::__invoke(__proj2, *__first2)))
		  return true;
		if (std::__invoke(__comp,
				  std::__invoke(__proj2, *__first2),
				  std::__invoke(__proj1, *__first1)))
		  return false;
	      }
	    return __first1 == __last1 && __first2 != __last2;
	  }
      }

    template<input_range _Range1, input_range _Range2,
	     typename _Proj1 = identity, typename _Proj2 = identity,
	     indirect_strict_weak_order<projected<iterator_t<_Range1>, _Proj1>,
					projected<iterator_t<_Range2>, _Proj2>>
	       _Comp = ranges::less>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Comp __comp = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__comp),
		       std::move(__proj1), std::move(__proj2));
      }

  private:
    template<typename _Iter, typename _Ref = iter_reference_t<_Iter>>
      static constexpr bool __ptr_to_nonvolatile
	= is_pointer_v<_Iter> && !is_volatile_v<remove_reference_t<_Ref>>;
  };

  inline constexpr __lexicographical_compare_fn lexicographical_compare;

  template<typename _Iter>
    struct in_found_result
    {
      [[no_unique_address]] _Iter in;
      bool found;

      template<typename _Iter2>
	requires convertible_to<const _Iter&, _Iter2>
	constexpr
	operator in_found_result<_Iter2>() const &
	{ return {in, found}; }

      template<typename _Iter2>
	requires convertible_to<_Iter, _Iter2>
	constexpr
	operator in_found_result<_Iter2>() &&
	{ return {std::move(in), found}; }
    };

  template<typename _Iter>
    using next_permutation_result = in_found_result<_Iter>;

  struct __next_permutation_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr next_permutation_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), false};

	auto __i = __first;
	++__i;
	if (__i == __last)
	  return {std::move(__i), false};

	auto __lasti = ranges::next(__first, __last);
	__i = __lasti;
	--__i;

	for (;;)
	  {
	    auto __ii = __i;
	    --__i;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__i),
			      std::__invoke(__proj, *__ii)))
	      {
		auto __j = __lasti;
		while (!(bool)std::__invoke(__comp,
					    std::__invoke(__proj, *__i),
					    std::__invoke(__proj, *--__j)))
		  ;
		ranges::iter_swap(__i, __j);
		ranges::reverse(__ii, __last);
		return {std::move(__lasti), true};
	      }
	    if (__i == __first)
	      {
		ranges::reverse(__first, __last);
		return {std::move(__lasti), false};
	      }
	  }
      }

    template<bidirectional_range _Range, typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr next_permutation_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __next_permutation_fn next_permutation{};

  template<typename _Iter>
    using prev_permutation_result = in_found_result<_Iter>;

  struct __prev_permutation_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Comp = ranges::less, typename _Proj = identity>
      requires sortable<_Iter, _Comp, _Proj>
      constexpr prev_permutation_result<_Iter>
      operator()(_Iter __first, _Sent __last,
		 _Comp __comp = {}, _Proj __proj = {}) const
      {
	if (__first == __last)
	  return {std::move(__first), false};

	auto __i = __first;
	++__i;
	if (__i == __last)
	  return {std::move(__i), false};

	auto __lasti = ranges::next(__first, __last);
	__i = __lasti;
	--__i;

	for (;;)
	  {
	    auto __ii = __i;
	    --__i;
	    if (std::__invoke(__comp,
			      std::__invoke(__proj, *__ii),
			      std::__invoke(__proj, *__i)))
	      {
		auto __j = __lasti;
		while (!(bool)std::__invoke(__comp,
					    std::__invoke(__proj, *--__j),
					    std::__invoke(__proj, *__i)))
		  ;
		ranges::iter_swap(__i, __j);
		ranges::reverse(__ii, __last);
		return {std::move(__lasti), true};
	      }
	    if (__i == __first)
	      {
		ranges::reverse(__first, __last);
		return {std::move(__lasti), false};
	      }
	  }
      }

    template<bidirectional_range _Range, typename _Comp = ranges::less,
	     typename _Proj = identity>
      requires sortable<iterator_t<_Range>, _Comp, _Proj>
      constexpr prev_permutation_result<borrowed_iterator_t<_Range>>
      operator()(_Range&& __r, _Comp __comp = {}, _Proj __proj = {}) const
      {
	return (*this)(ranges::begin(__r), ranges::end(__r),
		       std::move(__comp), std::move(__proj));
      }
  };

  inline constexpr __prev_permutation_fn prev_permutation{};

#if __glibcxx_ranges_contains >= 202207L // C++ >= 23
  struct __contains_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	    typename _Proj = identity,
	    typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<_Iter, _Proj>, const _Tp*>
      constexpr bool
      operator()(_Iter __first, _Sent __last, const _Tp& __value, _Proj __proj = {}) const
      { return ranges::find(std::move(__first), __last, __value, std::move(__proj)) != __last; }

    template<input_range _Range,
	    typename _Proj = identity,
	    typename _Tp
	      _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to,
					 projected<iterator_t<_Range>, _Proj>, const _Tp*>
      constexpr bool
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), __value, std::move(__proj)); }
  };

  inline constexpr __contains_fn contains{};

  struct __contains_subrange_fn
  {
    template<forward_iterator _Iter1, sentinel_for<_Iter1> _Sent1,
	     forward_iterator _Iter2, sentinel_for<_Iter2> _Sent2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<_Iter1, _Iter2, _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Iter1 __first1, _Sent1 __last1, _Iter2 __first2, _Sent2 __last2,
		 _Pred __pred = {}, _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return __first2 == __last2
	  || !ranges::search(__first1, __last1, __first2, __last2,
			     std::move(__pred), std::move(__proj1), std::move(__proj2)).empty();
      }

    template<forward_range _Range1, forward_range _Range2,
	     typename _Pred = ranges::equal_to,
	     typename _Proj1 = identity, typename _Proj2 = identity>
      requires indirectly_comparable<iterator_t<_Range1>, iterator_t<_Range2>,
				     _Pred, _Proj1, _Proj2>
      constexpr bool
      operator()(_Range1&& __r1, _Range2&& __r2, _Pred __pred = {},
		 _Proj1 __proj1 = {}, _Proj2 __proj2 = {}) const
      {
	return (*this)(ranges::begin(__r1), ranges::end(__r1),
		       ranges::begin(__r2), ranges::end(__r2),
		       std::move(__pred), std::move(__proj1), std::move(__proj2));
      }
  };

  inline constexpr __contains_subrange_fn contains_subrange{};

#endif // __glibcxx_ranges_contains

#if __glibcxx_ranges_find_last >= 202207L // C++ >= 23

  struct __find_last_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Proj = identity,
	     typename _Tp _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(_Iter, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to, projected<_Iter, _Proj>, const _Tp*>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, const _Tp& __value, _Proj __proj = {}) const
      {
	if constexpr (same_as<_Iter, _Sent> && bidirectional_iterator<_Iter>)
	  {
	    _Iter __found = ranges::find(reverse_iterator<_Iter>{__last},
					 reverse_iterator<_Iter>{__first},
					 __value, std::move(__proj)).base();
	    if (__found == __first)
	      return {__last, __last};
	    else
	      return {ranges::prev(__found), __last};
	  }
	else
	  {
	    _Iter __found = ranges::find(__first, __last, __value, __proj);
	    if (__found == __last)
	      return {__found, __found};
	    __first = __found;
	    for (;;)
	      {
		__first = ranges::find(ranges::next(__first), __last, __value, __proj);
		if (__first == __last)
		  return {__found, __first};
		__found = __first;
	      }
	  }
      }

    template<forward_range _Range, typename _Proj = identity,
	     typename _Tp
	       _GLIBCXX26_RANGE_ALGO_DEF_VAL_T(iterator_t<_Range>, _Proj)>
      requires indirect_binary_predicate<ranges::equal_to, projected<iterator_t<_Range>, _Proj>, const _Tp*>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, const _Tp& __value, _Proj __proj = {}) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), __value, std::move(__proj)); }
  };

  inline constexpr __find_last_fn find_last{};

  struct __find_last_if_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, _Pred __pred, _Proj __proj = {}) const
      {
	if constexpr (same_as<_Iter, _Sent> && bidirectional_iterator<_Iter>)
	  {
	    _Iter __found = ranges::find_if(reverse_iterator<_Iter>{__last},
					    reverse_iterator<_Iter>{__first},
					    std::move(__pred), std::move(__proj)).base();
	    if (__found == __first)
	      return {__last, __last};
	    else
	      return {ranges::prev(__found), __last};
	  }
	else
	  {
	    _Iter __found = ranges::find_if(__first, __last, __pred, __proj);
	    if (__found == __last)
	      return {__found, __found};
	    __first = __found;
	    for (;;)
	      {
		__first = ranges::find_if(ranges::next(__first), __last, __pred, __proj);
		if (__first == __last)
		  return {__found, __first};
		__found = __first;
	      }
	  }
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>> _Pred>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__pred), std::move(__proj)); }
  };

  inline constexpr __find_last_if_fn find_last_if{};

  struct __find_last_if_not_fn
  {
    template<forward_iterator _Iter, sentinel_for<_Iter> _Sent, typename _Proj = identity,
	     indirect_unary_predicate<projected<_Iter, _Proj>> _Pred>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, _Pred __pred, _Proj __proj = {}) const
      {
	if constexpr (same_as<_Iter, _Sent> && bidirectional_iterator<_Iter>)
	  {
	    _Iter __found = ranges::find_if_not(reverse_iterator<_Iter>{__last},
						reverse_iterator<_Iter>{__first},
						std::move(__pred), std::move(__proj)).base();
	    if (__found == __first)
	      return {__last, __last};
	    else
	      return {ranges::prev(__found), __last};
	  }
	else
	  {
	    _Iter __found = ranges::find_if_not(__first, __last, __pred, __proj);
	    if (__found == __last)
	      return {__found, __found};
	    __first = __found;
	    for (;;)
	      {
		__first = ranges::find_if_not(ranges::next(__first), __last, __pred, __proj);
		if (__first == __last)
		  return {__found, __first};
		__found = __first;
	      }
	  }
      }

    template<forward_range _Range, typename _Proj = identity,
	     indirect_unary_predicate<projected<iterator_t<_Range>, _Proj>> _Pred>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, _Pred __pred, _Proj __proj = {}) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__pred), std::move(__proj)); }
  };

  inline constexpr __find_last_if_not_fn find_last_if_not{};

#endif // __glibcxx_ranges_find_last

#if __glibcxx_ranges_fold >= 202207L // C++ >= 23

  template<typename _Iter, typename _Tp>
    struct in_value_result
    {
      [[no_unique_address]] _Iter in;
      [[no_unique_address]] _Tp value;

      template<typename _Iter2, typename _Tp2>
	requires convertible_to<const _Iter&, _Iter2>
	  && convertible_to<const _Tp&, _Tp2>
      constexpr
      operator in_value_result<_Iter2, _Tp2>() const &
      { return {in, value}; }

      template<typename _Iter2, typename _Tp2>
	requires convertible_to<_Iter, _Iter2>
	  && convertible_to<_Tp, _Tp2>
      constexpr
      operator in_value_result<_Iter2, _Tp2>() &&
      { return {std::move(in), std::move(value)}; }
    };

  namespace __detail
  {
    template<typename _Fp>
      class __flipped
      {
	_Fp _M_f;

      public:
	template<typename _Tp, typename _Up>
	  requires invocable<_Fp&, _Up, _Tp>
	invoke_result_t<_Fp&, _Up, _Tp>
	operator()(_Tp&&, _Up&&); // not defined
      };

      template<typename _Fp, typename _Tp, typename _Iter, typename _Up>
      concept __indirectly_binary_left_foldable_impl = movable<_Tp> && movable<_Up>
	&& convertible_to<_Tp, _Up>
	&& invocable<_Fp&, _Up, iter_reference_t<_Iter>>
	&& assignable_from<_Up&, invoke_result_t<_Fp&, _Up, iter_reference_t<_Iter>>>;

      template<typename _Fp, typename _Tp, typename _Iter>
      concept __indirectly_binary_left_foldable = copy_constructible<_Fp>
	&& indirectly_readable<_Iter>
	&& invocable<_Fp&, _Tp, iter_reference_t<_Iter>>
	&& convertible_to<invoke_result_t<_Fp&, _Tp, iter_reference_t<_Iter>>,
			  decay_t<invoke_result_t<_Fp&, _Tp, iter_reference_t<_Iter>>>>
	&& __indirectly_binary_left_foldable_impl
	    <_Fp, _Tp, _Iter, decay_t<invoke_result_t<_Fp&, _Tp, iter_reference_t<_Iter>>>>;

      template <typename _Fp, typename _Tp, typename _Iter>
      concept __indirectly_binary_right_foldable
	= __indirectly_binary_left_foldable<__flipped<_Fp>, _Tp, _Iter>;
  } // namespace __detail

  template<typename _Iter, typename _Tp>
    using fold_left_with_iter_result = in_value_result<_Iter, _Tp>;

  struct __fold_left_with_iter_fn
  {
    template<typename _Ret_iter,
	     typename _Iter, typename _Sent, typename _Tp, typename _Fp>
      static constexpr auto
      _S_impl(_Iter __first, _Sent __last, _Tp __init, _Fp __f)
      {
	using _Up = decay_t<invoke_result_t<_Fp&, _Tp, iter_reference_t<_Iter>>>;
	using _Ret = fold_left_with_iter_result<_Ret_iter, _Up>;

	if (__first == __last)
	  return _Ret{std::move(__first), _Up(std::move(__init))};

	_Up __accum = std::__invoke(__f, std::move(__init), *__first);
	for (++__first; __first != __last; ++__first)
	  __accum = std::__invoke(__f, std::move(__accum), *__first);
	return _Ret{std::move(__first), std::move(__accum)};
      }

    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(iter_value_t<_Iter>),
	     __detail::__indirectly_binary_left_foldable<_Tp, _Iter> _Fp>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Tp __init, _Fp __f) const
      {
	using _Ret_iter = _Iter;
	return _S_impl<_Ret_iter>(std::move(__first), __last,
				  std::move(__init), std::move(__f));
      }

    template<input_range _Range,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(range_value_t<_Range>),
	     __detail::__indirectly_binary_left_foldable<_Tp, iterator_t<_Range>> _Fp>
      constexpr auto
      operator()(_Range&& __r, _Tp __init, _Fp __f) const
      {
	using _Ret_iter = borrowed_iterator_t<_Range>;
	return _S_impl<_Ret_iter>(ranges::begin(__r), ranges::end(__r),
				  std::move(__init), std::move(__f));
      }
  };

  inline constexpr __fold_left_with_iter_fn fold_left_with_iter{};

  struct __fold_left_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(iter_value_t<_Iter>),
	     __detail::__indirectly_binary_left_foldable<_Tp, _Iter> _Fp>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Tp __init, _Fp __f) const
      {
	return ranges::fold_left_with_iter(std::move(__first), __last,
					   std::move(__init), std::move(__f)).value;
      }

    template<input_range _Range,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(range_value_t<_Range>),
	     __detail::__indirectly_binary_left_foldable<_Tp, iterator_t<_Range>> _Fp>
      constexpr auto
      operator()(_Range&& __r, _Tp __init, _Fp __f) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__init), std::move(__f)); }
  };

  inline constexpr __fold_left_fn fold_left{};

  template<typename _Iter, typename _Tp>
    using fold_left_first_with_iter_result = in_value_result<_Iter, _Tp>;

  struct __fold_left_first_with_iter_fn
  {
    template<typename _Ret_iter, typename _Iter, typename _Sent, typename _Fp>
      static constexpr auto
      _S_impl(_Iter __first, _Sent __last, _Fp __f)
      {
	using _Up = decltype(ranges::fold_left(std::move(__first), __last,
					       iter_value_t<_Iter>(*__first), __f));
	using _Ret = fold_left_first_with_iter_result<_Ret_iter, optional<_Up>>;

	if (__first == __last)
	  return _Ret{std::move(__first), optional<_Up>()};

	optional<_Up> __init(in_place, *__first);
	for (++__first; __first != __last; ++__first)
	  *__init = std::__invoke(__f, std::move(*__init), *__first);
	return _Ret{std::move(__first), std::move(__init)};
      }

    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     __detail::__indirectly_binary_left_foldable<iter_value_t<_Iter>, _Iter> _Fp>
      requires constructible_from<iter_value_t<_Iter>, iter_reference_t<_Iter>>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Fp __f) const
      {
	using _Ret_iter = _Iter;
	return _S_impl<_Ret_iter>(std::move(__first), __last, std::move(__f));
      }

    template<input_range _Range,
	     __detail::__indirectly_binary_left_foldable<range_value_t<_Range>, iterator_t<_Range>> _Fp>
      requires constructible_from<range_value_t<_Range>, range_reference_t<_Range>>
      constexpr auto
      operator()(_Range&& __r, _Fp __f) const
      {
	using _Ret_iter = borrowed_iterator_t<_Range>;
	return _S_impl<_Ret_iter>(ranges::begin(__r), ranges::end(__r), std::move(__f));
      }
  };

  inline constexpr __fold_left_first_with_iter_fn fold_left_first_with_iter{};

  struct __fold_left_first_fn
  {
    template<input_iterator _Iter, sentinel_for<_Iter> _Sent,
	     __detail::__indirectly_binary_left_foldable<iter_value_t<_Iter>, _Iter> _Fp>
      requires constructible_from<iter_value_t<_Iter>, iter_reference_t<_Iter>>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Fp __f) const
      {
	return ranges::fold_left_first_with_iter(std::move(__first), __last,
						 std::move(__f)).value;
      }

    template<input_range _Range,
	     __detail::__indirectly_binary_left_foldable<range_value_t<_Range>, iterator_t<_Range>> _Fp>
      requires constructible_from<range_value_t<_Range>, range_reference_t<_Range>>
      constexpr auto
      operator()(_Range&& __r, _Fp __f) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__f)); }
  };

  inline constexpr __fold_left_first_fn fold_left_first{};

  struct __fold_right_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(iter_value_t<_Iter>),
	     __detail::__indirectly_binary_right_foldable<_Tp, _Iter> _Fp>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Tp __init, _Fp __f) const
      {
	using _Up = decay_t<invoke_result_t<_Fp&, iter_reference_t<_Iter>, _Tp>>;

	if (__first == __last)
	  return _Up(std::move(__init));

	_Iter __tail = ranges::next(__first, __last);
	_Up __accum = std::__invoke(__f, *--__tail, std::move(__init));
	while (__first != __tail)
	  __accum = std::__invoke(__f, *--__tail, std::move(__accum));
	return __accum;
      }

    template<bidirectional_range _Range,
	     typename _Tp _GLIBCXX26_DEF_VAL_T(range_value_t<_Range>),
	     __detail::__indirectly_binary_right_foldable<_Tp, iterator_t<_Range>> _Fp>
      constexpr auto
      operator()(_Range&& __r, _Tp __init, _Fp __f) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__init), std::move(__f)); }
  };

  inline constexpr __fold_right_fn fold_right{};

  struct __fold_right_last_fn
  {
    template<bidirectional_iterator _Iter, sentinel_for<_Iter> _Sent,
	     __detail::__indirectly_binary_right_foldable<iter_value_t<_Iter>, _Iter> _Fp>
      requires constructible_from<iter_value_t<_Iter>, iter_reference_t<_Iter>>
      constexpr auto
      operator()(_Iter __first, _Sent __last, _Fp __f) const
      {
	using _Up = decltype(ranges::fold_right(__first, __last,
						iter_value_t<_Iter>(*__first), __f));

	if (__first == __last)
	  return optional<_Up>();

	_Iter __tail = ranges::prev(ranges::next(__first, std::move(__last)));
	return optional<_Up>(in_place,
			     ranges::fold_right(std::move(__first), __tail,
						iter_value_t<_Iter>(*__tail),
						std::move(__f)));
      }

    template<bidirectional_range _Range,
	     __detail::__indirectly_binary_right_foldable<range_value_t<_Range>, iterator_t<_Range>> _Fp>
      requires constructible_from<range_value_t<_Range>, range_reference_t<_Range>>
      constexpr auto
      operator()(_Range&& __r, _Fp __f) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), std::move(__f)); }
  };

  inline constexpr __fold_right_last_fn fold_right_last{};
#endif // __glibcxx_ranges_fold
} // namespace ranges

#if __glibcxx_shift >= 201806L // C++ >= 20
  template<typename _ForwardIterator>
    constexpr _ForwardIterator
    shift_left(_ForwardIterator __first, _ForwardIterator __last,
	       typename iterator_traits<_ForwardIterator>::difference_type __n)
    {
      __glibcxx_assert(__n >= 0);
      if (__n == 0)
	return __last;

      auto __mid = ranges::next(__first, __n, __last);
      if (__mid == __last)
	return __first;
      return std::move(std::move(__mid), std::move(__last), std::move(__first));
    }

  template<typename _ForwardIterator>
    constexpr _ForwardIterator
    shift_right(_ForwardIterator __first, _ForwardIterator __last,
		typename iterator_traits<_ForwardIterator>::difference_type __n)
    {
      __glibcxx_assert(__n >= 0);
      if (__n == 0)
	return __first;

      using _Cat
	= typename iterator_traits<_ForwardIterator>::iterator_category;
      if constexpr (derived_from<_Cat, bidirectional_iterator_tag>)
	{
	  auto __mid = ranges::next(__last, -__n, __first);
	  if (__mid == __first)
	    return __last;

	  return std::move_backward(std::move(__first), std::move(__mid),
				    std::move(__last));
	}
      else
	{
	  auto __result = ranges::next(__first, __n, __last);
	  if (__result == __last)
	    return __last;

	  auto __dest_head = __first, __dest_tail = __result;
	  while (__dest_head != __result)
	    {
	      if (__dest_tail == __last)
		{
		  // If we get here, then we must have
		  //     2*n >= distance(__first, __last)
		  // i.e. we are shifting out at least half of the range.  In
		  // this case we can safely perform the shift with a single
		  // move.
		  std::move(std::move(__first), std::move(__dest_head), __result);
		  return __result;
		}
	      ++__dest_head;
	      ++__dest_tail;
	    }

	  for (;;)
	    {
	      // At the start of each iteration of this outer loop, the range
	      // [__first, __result) contains those elements that after shifting
	      // the whole range right by __n, should end up in
	      // [__dest_head, __dest_tail) in order.

	      // The below inner loop swaps the elements of [__first, __result)
	      // and [__dest_head, __dest_tail), while simultaneously shifting
	      // the latter range by __n.
	      auto __cursor = __first;
	      while (__cursor != __result)
		{
		  if (__dest_tail == __last)
		    {
		      // At this point the ranges [__first, result) and
		      // [__dest_head, dest_tail) are disjoint, so we can safely
		      // move the remaining elements.
		      __dest_head = std::move(__cursor, __result,
					      std::move(__dest_head));
		      std::move(std::move(__first), std::move(__cursor),
				std::move(__dest_head));
		      return __result;
		    }
		  std::iter_swap(__cursor, __dest_head);
		  ++__dest_head;
		  ++__dest_tail;
		  ++__cursor;
		}
	    }
	}
    }
#endif

namespace ranges
{
#if __glibcxx_shift >= 202202L // C++ >= 23
  struct __shift_left_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, iter_difference_t<_Iter> __n) const
      {
	__glibcxx_assert(__n >= 0);
	if (__n == 0)
	  return {__first, ranges::next(__first, __last)};

	auto __mid = ranges::next(__first, __n, __last);
	if (__mid == __last)
	  return {__first, __first};
	return {__first, ranges::move(__mid, __last, __first).out};
      }

    template<forward_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, range_difference_t<_Range> __n) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), __n); }
  };

  inline constexpr __shift_left_fn shift_left{};

  struct __shift_right_fn
  {
    template<permutable _Iter, sentinel_for<_Iter> _Sent>
      constexpr subrange<_Iter>
      operator()(_Iter __first, _Sent __last, iter_difference_t<_Iter> __n) const
      {
	__glibcxx_assert(__n >= 0);
	if (__n == 0)
	  return {__first, ranges::next(__first, __last)};

	if constexpr (bidirectional_iterator<_Iter> && same_as<_Iter, _Sent>)
	  {
	    auto __mid = ranges::next(__last, -__n, __first);
	    if (__mid == __first)
	      return {__last, __last};

	    return {ranges::move_backward(__first, __mid, __last).out, __last};
	  }
	else
	  {
	    auto __result = ranges::next(__first, __n, __last);
	    if (__result == __last)
	      return {__result, __result};

	    auto __dest_head = __first, __dest_tail = __result;
	    while (__dest_head != __result)
	      {
		if (__dest_tail == __last)
		  {
		    // If we get here, then we must have
		    //     2*n >= distance(__first, __last)
		    // i.e. we are shifting out at least half of the range.  In
		    // this case we can safely perform the shift with a single
		    // move.
		    auto __lasti = ranges::move(__first, __dest_head, __result).out;
		    // __glibcxx_assert(__lasti == __last);
		    return {__result, __lasti};
		  }
		++__dest_head;
		++__dest_tail;
	      }

	    for (;;)
	      {
		// At the start of each iteration of this outer loop, the range
		// [__first, __result) contains those elements that after shifting
		// the whole range right by __n, should end up in
		// [__dest_head, __dest_tail) in order.

		// The below inner loop swaps the elements of [__first, __result)
		// and [__dest_head, __dest_tail), while simultaneously shifting
		// the latter range by __n.
		auto __cursor = __first;
		while (__cursor != __result)
		  {
		    if (__dest_tail == __last)
		      {
			// At this point the ranges [__first, result) and
			// [__dest_head, dest_tail) are disjoint, so we can safely
			// move the remaining elements.
			__dest_head = ranges::move(__cursor, __result, __dest_head).out;
			auto __lasti = ranges::move(__first, __cursor, __dest_head).out;
			// __glibcxx_assert(__lasti == __last);
			return {__result, __lasti};
		      }
		    ranges::iter_swap(__cursor, __dest_head);
		    ++__dest_head;
		    ++__dest_tail;
		    ++__cursor;
		  }
	      }
	  }
      }

    template<forward_range _Range>
      requires permutable<iterator_t<_Range>>
      constexpr borrowed_subrange_t<_Range>
      operator()(_Range&& __r, range_difference_t<_Range> __n) const
      { return (*this)(ranges::begin(__r), ranges::end(__r), __n); }
  };

  inline constexpr __shift_right_fn shift_right{};
#endif // C++23
} // namespace ranges

_GLIBCXX_END_NAMESPACE_VERSION
} // namespace std
#endif // concepts
#endif // C++20
#endif // _RANGES_ALGO_H
