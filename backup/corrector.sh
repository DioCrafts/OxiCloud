#!/bin/bash
# Realiza una copia de seguridad antes de ejecutar cambios automáticos
find . -type f -name "*.rs" | while read file; do
    sed -i.bak 's/"\\OC\\User"/r"\\OC\\User"/g' "$file"
done

