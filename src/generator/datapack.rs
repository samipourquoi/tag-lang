use crate::generator::Generator;
use crate::parser::expression::VariableName;
use crate::generator::simplify::Simplify;
use crate::errors::CompilerError;
use crate::CompileOptions;

impl Generator {
    pub fn write_datapack(&mut self, options: CompileOptions) -> Result<(), CompilerError> {
        let pack_mcmeta = self.generate_pack_mcmeta()?;
        let functions_dir = options.outdir
            .join("data")
            .join(options.namespace)
            .join("functions");
        self.pop_scope();
        std::fs::create_dir_all(&functions_dir);
        std::fs::write(options.outdir.join("pack.mcmeta"), pack_mcmeta);
        for (name, content) in &self.files {
            std::fs::write(functions_dir.join(format!("{}.mcfunction", name)), content.join("\n"));
        }

        Ok(())
    }

    fn generate_pack_mcmeta(&self) -> Result<String, CompilerError> {
        let pack_format: String
            = self.get_static_variable_value(&VariableName::Static("pack_format".into()))
            .expect("variable `pack_format` isn't set to any value")
            .to_string(self)?;
        let pack_description: String
            = self.get_static_variable_value(&VariableName::Static("pack_description".into()))
            .expect("variable `pack_description` isn't set to any value")
            .to_string(self)?;
        Ok(format!(include_str!("../data/pack.mcmeta"), pack_format, pack_description))
    }
}
