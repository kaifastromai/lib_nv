find ./ -type f -name "*.rs" -exec wc -l {} \; | awk '{total += $1} END{print total}'
