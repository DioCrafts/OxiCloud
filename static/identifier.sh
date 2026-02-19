while IFS= read -r -d '' file; do
    if grep -Iq . "$file"; then
        echo "===== $file ====="
        cat "$file"
        echo -e "\n"
    fi
done < <(find . -type f -print0)

