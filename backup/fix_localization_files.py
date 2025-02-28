#!/usr/bin/env python3
"""
Script específico para corregir archivos de localización que tienen problemas
con cadenas y comillas en los archivos eo.rs y lv.rs.
"""

import os
import re
import sys
from pathlib import Path
import shutil

def fix_localization_file(file_path):
    """Repara un archivo de localización específico"""
    print(f"Procesando {file_path}...")
    
    try:
        with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
            
        original_content = content
        fixed_count = 0
        
        # 1. Reemplazar cadenas con comillas mal formateadas
        # Ejemplo: "Datumbaza eraro: "%s"" -> r"Datumbaza eraro: "%s""
        matches = re.finditer(r'"([^"]*)"([^"]*)"', content)
        for match in matches:
            full_match = match.group(0)
            # Verificar si esto parece una cadena con comillas internas
            if "%s" in full_match or "@" in full_match:
                # Usar cadena raw (r"") para evitar problemas con comillas internas
                replacement = r'r' + full_match
                content = content.replace(full_match, replacement)
                fixed_count += 1
        
        # 2. Arreglar específicamente patrones conocidos en eo.rs
        patterns = [
            (r'"Datumbaza eraro: "%s""', r'r"Datumbaza eraro: \"%s\""'),
            (r'"La uzanto de MySQL "%s"@"localhost" jam ekzistas."', r'r"La uzanto de MySQL \"%s\"@\"localhost\" jam ekzistas."'),
            (r'"La uzanto de MySQL "%s"@"%%" jam ekzistas"', r'r"La uzanto de MySQL \"%s\"@\"%%" jam ekzistas"'),
            (r'"Ne troviĝis kategorio "%s""', r'r"Ne troviĝis kategorio \"%s\""')
        ]
        
        for old_str, new_str in patterns:
            if old_str in content:
                content = content.replace(old_str, new_str)
                fixed_count += 1
        
        # 3. Arreglar específicamente patrones conocidos en lv.rs
        patterns = [
            (r'"DB kļūda — "%s""', r'r"DB kļūda — \"%s\""'),
            (r'"Vainīgā komanda bija "%s""', r'r"Vainīgā komanda bija \"%s\""'),
            (r'"Nevarēja atrast kategoriju "%s""', r'r"Nevarēja atrast kategoriju \"%s\""')
        ]
        
        for old_str, new_str in patterns:
            if old_str in content:
                content = content.replace(old_str, new_str)
                fixed_count += 1
        
        # 4. Asegurarse de que todas las comillas estén bien formateadas
        content = re.sub(r'@"([^"]*)"', r'@"\1"', content)
        
        # Verificar si se hicieron cambios
        if content != original_content:
            # Hacer backup del archivo original
            backup_path = file_path.with_suffix('.rs.bak')
            shutil.copy2(file_path, backup_path)
            print(f"  Backup creado en {backup_path}")
            
            # Guardar el contenido corregido
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            
            print(f"  ✓ Corregidas {fixed_count} cadenas de texto en {file_path}")
            return True
        else:
            print(f"  No se encontraron problemas en {file_path}")
            return False
    
    except Exception as e:
        print(f"  ✗ Error procesando {file_path}: {str(e)}")
        return False

def main():
    """Función principal"""
    if len(sys.argv) < 2:
        print("Uso: python fix_localization_files.py <directorio_proyecto>")
        print("     Especifique la ruta al directorio raíz del proyecto Rust")
        sys.exit(1)
    
    project_dir = Path(sys.argv[1])
    
    # Verificar que el directorio existe
    if not project_dir.exists():
        print(f"Error: El directorio {project_dir} no existe")
        sys.exit(1)
    
    # Buscar archivos de localización específicos
    files_to_fix = []
    
    # Primero buscar los archivos que sabemos que tienen problemas
    eo_rs = list(project_dir.glob("**/eo.rs"))
    lv_rs = list(project_dir.glob("**/lv.rs"))
    
    files_to_fix.extend(eo_rs)
    files_to_fix.extend(lv_rs)
    
    if not files_to_fix:
        # Si no se encontraron los archivos específicos, buscar en todos los archivos de l10n
        files_to_fix = list(project_dir.glob("**/l10n/*.rs"))
    
    if not files_to_fix:
        print("No se encontraron archivos de localización para corregir")
        sys.exit(0)
    
    # Corregir los archivos
    fixed_count = 0
    for file_path in files_to_fix:
        if fix_localization_file(file_path):
            fixed_count += 1
    
    print(f"\nProceso completado. {fixed_count}/{len(files_to_fix)} archivos corregidos.")

if __name__ == "__main__":
    main()
