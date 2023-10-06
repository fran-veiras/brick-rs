#!/bin/bash

# Copia el archivo desde /target/debug/brick a /test-brick
cp -f target/debug/brick test-brick/

# Comprueba si la copia fue exitosa
if [ $? -eq 0 ]; then
  echo "Archivo copiado exitosamente."
else
  echo "Error al copiar el archivo."
fi

