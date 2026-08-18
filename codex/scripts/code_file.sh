#!/bin/bash

if [[ "$1" == "1" ]]; then
    code ./src/variables_and_mutability.rs
elif [[ "$1" == "2" ]]; then
    code ./src/compound_data_type.rs
elif [[ "$1" == "3" ]]; then
    code ./src/function.rs
elif [[ "$1" == "4" ]]; then
    code ./src/control_flow.rs
elif [[ "$1" == "5" ]]; then
    code ./src/references.rs
elif [[ "$1" == "6" ]]; then
    code ./src/structure.rs
elif [[ "$1" == "7" ]]; then
    code ./src/enum_option.rs
fi