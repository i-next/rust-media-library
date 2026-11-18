mkdir -p src/{ui,database,models,services}

touch \
src/main.rs \
src/ui/{mod.rs,app.rs} \
src/database/{mod.rs,sqlite.rs} \
src/models/media.rs \
src/services/scanner.rs