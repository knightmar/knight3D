make clean && make && prime-run ./bin/opengl_app > log.log 2>&1
grep -E 'DEBUG: Processing face line:|DEBUG: Token:|DEBUG: sscanf result:|DEBUG: Storing 0-based index:|OBJ Parser:.*(Warning|Error|Skipping)' log.log | head -n 50
