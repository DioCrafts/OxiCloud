#!/usr/bin/env python3
"""
Script para corregir errores comunes en la migración de código PHP a Rust.
Específicamente aborda problemas de:
1. Corrección de cadenas en archivos de localización
2. Conversión de comentarios de documentación mal formateados
3. Corrección de caracteres de escape en cadenas
4. Solución para módulos tipo "type"
5. Corrección de llaves faltantes
"""

import os
import re
import sys
from pathlib import Path
import shutil

class RustFixers:
    def __init__(self, project_dir, dry_run=False):
        self.project_dir = Path(project_dir)
        self.dry_run = dry_run
        self.fixed_files = 0
        self.fixed_issues = 0
        
        # Verificar que el directorio existe
        if not self.project_dir.exists():
            print(f"Error: El directorio {project_dir} no existe")
            sys.exit(1)
    
    def run_all_fixes(self):
        """Ejecuta todas las correcciones"""
        print(f"Analizando proyecto en {self.project_dir}")
        
        # 1. Corregir problemas en archivos de localización
        self.fix_localization_files()
        
        # 2. Convertir comentarios de documentación mal formateados
        self.fix_doc_comments()
        
        # 3. Corregir caracteres de escape en cadenas
        self.fix_escape_sequences()
        
        # 4. Corregir módulo "type" (palabra reservada)
        self.fix_type_modules()
        
        # 5. Corregir llaves sin cerrar
        self.fix_unclosed_braces()
        
        # 6. Corregir constantes en struct
        self.fix_struct_constants()
        
        # 7. Corregir errores de "protected fn" (no existe en Rust)
        self.fix_protected_methods()
        
        print(f"\nResumen de correcciones:")
        print(f"- Archivos modificados: {self.fixed_files}")
        print(f"- Problemas corregidos: {self.fixed_issues}")
        
        if self.dry_run:
            print("Modo prueba - No se realizaron cambios reales")
    
    def fix_localization_files(self):
        """Corrige problemas en archivos de localización"""
        print("\nBuscando problemas en archivos de localización...")
        target_files = self.project_dir.glob("**/l10n/*.rs")
        
        for file_path in target_files:
            try:
                content = file_path.read_text(encoding='utf-8', errors='replace')
                original_content = content
                modified = False
                
                # 1. Corregir cadenas con comillas no escapadas: "Datumbaza eraro: "%s""
                pattern = r'"([^"]*)"([^"]*)"([^"]*)"'
                replacement = r'"$1\\"$2\\"$3"'
                new_content = re.sub(pattern, replacement, content)
                
                # 2. Usar r"" para cadenas con _ en prefijo
                pattern = r'"([^"]*_%[^"]*)"'
                replacement = r'r"\1"'
                new_content = re.sub(pattern, new_content)
                
                # 3. Corregir cadenas de localización con comillas internas
                pattern = r'(".*?)"%s(.*?")'
                replacement = r'\1"%s\2'
                new_content = re.sub(pattern, replacement, new_content)
                
                # Verificar si hubo cambios
                if new_content != original_content:
                    modified = True
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        file_path.write_text(new_content)
                    
                    self.fixed_issues += sum(1 for _ in re.finditer(pattern, original_content))
                    print(f"  ✓ Corregidas cadenas en {file_path}")
                
                if modified:
                    self.fixed_files += 1
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_doc_comments(self):
        """Convierte comentarios de documentación //! a // normales cuando están fuera de contexto"""
        print("\nBuscando comentarios de documentación mal formateados...")
        
        # Buscar archivos Rust con posibles comentarios mal formateados
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
                    lines = f.readlines()
                
                modified = False
                new_lines = []
                in_item_block = False  # Para rastrear si estamos dentro de un bloque de ítems
                
                for i, line in enumerate(lines):
                    # Si la línea comienza con una declaración de ítem (struct, impl, fn, etc.)
                    if re.match(r'^\s*(pub\s+)?(struct|enum|fn|impl|trait|mod|use|const|static)', line):
                        in_item_block = True
                    
                    # Si la línea termina con }, probablemente estamos saliendo de un bloque
                    if re.search(r'}\s*$', line):
                        in_item_block = False
                    
                    # Si encontramos un comentario //! fuera de un contexto válido, convertirlo a //
                    if not in_item_block and line.strip().startswith('//!'):
                        # Si es una línea en blanco, simplificar
                        if line.strip() == '//!':
                            new_line = line.replace('//!', '//')
                        else:
                            new_line = line.replace('//!', '//')
                        
                        new_lines.append(new_line)
                        modified = True
                    else:
                        new_lines.append(line)
                
                if modified:
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.writelines(new_lines)
                    
                    self.fixed_files += 1
                    self.fixed_issues += 1
                    print(f"  ✓ Corregidos comentarios en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_escape_sequences(self):
        """Corrige secuencias de escape desconocidas en las cadenas de texto"""
        print("\nBuscando secuencias de escape incorrectas...")
        
        # Buscar archivos Rust con posibles secuencias de escape incorrectas
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                content = file_path.read_text(encoding='utf-8', errors='replace')
                original_content = content
                
                # Buscar patrones como "\OC\User" y reemplazarlos con r"\OC\User"
                pattern = r'"\\OC\\User"'
                replacement = r'r"\OC\User"'
                
                new_content = re.sub(pattern, replacement, content)
                
                # Verificar si hubo cambios
                if new_content != original_content:
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        file_path.write_text(new_content)
                    
                    self.fixed_files += 1
                    self.fixed_issues += sum(1 for _ in re.finditer(pattern, original_content))
                    print(f"  ✓ Corregidas secuencias de escape en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_type_modules(self):
        """Corrige declaraciones de módulos con nombres reservados como 'type'"""
        print("\nBuscando módulos con nombres reservados...")
        
        # Buscar archivos Rust con posibles declaraciones incorrectas
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                content = file_path.read_text(encoding='utf-8', errors='replace')
                original_content = content
                
                # Buscar declaraciones 'mod type' o 'pub mod type'
                pattern = r'(pub\s+)?mod\s+type\s*;'
                replacement = r'\1mod r#type;'
                
                new_content = re.sub(pattern, replacement, content)
                
                # Verificar si hubo cambios
                if new_content != original_content:
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        file_path.write_text(new_content)
                    
                    self.fixed_files += 1
                    self.fixed_issues += sum(1 for _ in re.finditer(pattern, original_content))
                    print(f"  ✓ Corregido 'mod type' en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_unclosed_braces(self):
        """Corrige archivos con llaves sin cerrar"""
        print("\nBuscando archivos con llaves sin cerrar...")
        
        # Buscar archivos Rust que puedan tener llaves sin cerrar
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                content = file_path.read_text(encoding='utf-8', errors='replace')
                
                # Contar llaves abiertas y cerradas
                open_braces = content.count('{')
                close_braces = content.count('}')
                
                if open_braces > close_braces:
                    missing_braces = open_braces - close_braces
                    
                    # Añadir llaves faltantes al final del archivo
                    new_content = content.rstrip() + '\n' + ('}' * missing_braces) + ' // Añadido por reparador automático\n'
                    
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        file_path.write_text(new_content)
                    
                    self.fixed_files += 1
                    self.fixed_issues += missing_braces
                    print(f"  ✓ Añadidas {missing_braces} llaves faltantes en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_struct_constants(self):
        """Corrige constantes declaradas dentro de structs (no permitido en Rust)"""
        print("\nBuscando constantes incorrectas en structs...")
        
        # Buscar archivos Rust que puedan tener constantes en structs
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                with open(file_path, 'r', encoding='utf-8', errors='replace') as f:
                    content = f.read()
                
                # Buscar patrones como "pub const X: T = valor;" dentro de un struct
                pattern = r'(struct\s+\w+\s*\{[^}]*?)(pub\s+const\s+\w+\s*:\s*[^;]+;)([^}]*\})'
                
                def replace_const(match):
                    struct_start = match.group(1)
                    const_decl = match.group(2)
                    struct_end = match.group(3)
                    
                    # Convertir la constante a un campo normal
                    const_field = re.sub(r'pub\s+const\s+(\w+)\s*:\s*([^=]+)=\s*([^;]+);', 
                                         r'pub \1: \2,  // Convertido de const', 
                                         const_decl)
                    
                    return struct_start + const_field + struct_end
                
                new_content = re.sub(pattern, replace_const, content)
                
                # Verificar si hubo cambios
                if new_content != content:
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        with open(file_path, 'w', encoding='utf-8') as f:
                            f.write(new_content)
                    
                    self.fixed_files += 1
                    self.fixed_issues += sum(1 for _ in re.finditer(pattern, content))
                    print(f"  ✓ Convertidas constantes en struct en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")
    
    def fix_protected_methods(self):
        """Corrige métodos marcados como 'protected' (no existe en Rust)"""
        print("\nBuscando métodos 'protected' (concepto de PHP)...")
        
        # Buscar archivos Rust que puedan tener métodos protected
        target_files = self.project_dir.glob("**/*.rs")
        
        for file_path in target_files:
            try:
                content = file_path.read_text(encoding='utf-8', errors='replace')
                original_content = content
                
                # Buscar declaraciones 'protected fn'
                pattern = r'protected\s+fn'
                replacement = r'pub(crate) fn'  # Alternativa más cercana en Rust
                
                new_content = re.sub(pattern, replacement, content)
                
                # Verificar si hubo cambios
                if new_content != original_content:
                    if not self.dry_run:
                        # Hacer copia de seguridad
                        backup_path = file_path.with_suffix('.rs.bak')
                        shutil.copy2(file_path, backup_path)
                        
                        # Escribir contenido corregido
                        file_path.write_text(new_content)
                    
                    self.fixed_files += 1
                    self.fixed_issues += sum(1 for _ in re.finditer(pattern, original_content))
                    print(f"  ✓ Reemplazado 'protected fn' por 'pub(crate) fn' en {file_path}")
            
            except Exception as e:
                print(f"  ✗ Error procesando {file_path}: {str(e)}")

def main():
    # Obtener directorio del proyecto
    if len(sys.argv) > 1:
        project_dir = sys.argv[1]
    else:
        project_dir = "."
    
    # Comprobar el modo dry-run
    dry_run = "--dry-run" in sys.argv
    
    # Crear y ejecutar el reparador
    fixer = RustFixers(project_dir, dry_run)
    fixer.run_all_fixes()
    
    print("\nProceso completado. Ejecuta 'cargo check' para verificar los resultados.")

if __name__ == "__main__":
    main()
