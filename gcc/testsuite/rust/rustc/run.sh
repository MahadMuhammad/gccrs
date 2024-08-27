#!/usr/bin/env bash

process_files() {
    # Recursively process rust files
    for file in "$1"/*.rs; do
        if [[ -f "$file" ]]; then
            base_name="${file%.rs}"
            stderr_file="${base_name}.stderr"
            # output_file="dg_${base_name##*/}.rs"
            output_file="${base_name}_dg.rs"

            # if we have `.stderr` file
            if [[ -f "$stderr_file" ]]; then
                rusttest-to-dg --file "$file" --stderr "$stderr_file" > "$output_file"
            else
                rusttest-to-dg --file "$file" > "$output_file"
            fi
        fi
    done

    # Recursively process subdirectories
    for dir in "$1"/*/; do
        if [[ -d "$dir" ]]; then
            process_files "$dir"
        fi
    done
}

process_files "$(pwd)"
