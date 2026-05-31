use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::builder::Builder;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::OptimizationLevel;

use crate::parser::ast::*;

pub struct Codegen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx> Codegen<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        Codegen { context, module, builder }
    }

    pub fn generate(&self, program: &Program) -> Result<(), String> {
        for stmt in &program.statements {
            self.gen_stmt(stmt)?;
        }
        Ok(())
    }

    fn gen_stmt(&self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::FunctionDecl(f) => self.gen_function(f),
            Stmt::ExprStatement(e) => {
                self.gen_expr(e)?;
                Ok(())
            }
        }
    }

    fn gen_function(&self, func: &FunctionDecl) -> Result<(), String> {
        let i32_type = self.context.i32_type();
        let fn_type = i32_type.fn_type(&[], false);
        let function = self.module.add_function(&func.name, fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);

        for stmt in &func.body {
            self.gen_stmt(stmt)?;
        }

        let zero = i32_type.const_int(0, false);
        self.builder.build_return(Some(&zero))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn gen_expr(&self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::FunctionCall(call) => self.gen_call(call),
            Expr::StringLiteral(_) => Ok(()),
        }
    }

    fn gen_call(&self, call: &FunctionCall) -> Result<(), String> {
        match call.name.as_str() {
            "print" => self.gen_print(call),
            name => Err(format!("Unknown function '{}'", name)),
        }
    }

    fn gen_print(&self, call: &FunctionCall) -> Result<(), String> {
        if call.args.len() != 1 {
            return Err("print() takes exactly 1 argument".to_string());
        }

        let Expr::StringLiteral(text) = &call.args[0] else {
            return Err("print() argument must be a string literal in Milestone 1".to_string());
        };

        let text_with_newline = format!("{}\n\0", text);

        let printf = self.get_or_declare_printf();

        let string_val = self.context.const_string(
            text_with_newline.as_bytes(),
            false,
        );
        let global = self.module.add_global(
            string_val.get_type(),
            None,
            "str",
        );
        global.set_initializer(&string_val);
        global.set_constant(true);

        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context.i32_type();
        let zero = i32_type.const_int(0, false);
        let ptr = unsafe {
            self.builder.build_gep(
                string_val.get_type(),
                global.as_pointer_value(),
                &[zero, zero],
                "str_ptr",
            ).map_err(|e| e.to_string())?
        };

        let ptr_cast = self.builder
            .build_pointer_cast(ptr, i8_ptr_type, "cast")
            .map_err(|e| e.to_string())?;

        self.builder
            .build_call(printf, &[ptr_cast.into()], "printf_call")
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    fn get_or_declare_printf(&self) -> inkwell::values::FunctionValue<'ctx> {
        if let Some(f) = self.module.get_function("printf") {
            return f;
        }

        let i8_ptr_type = self.context.i8_type().ptr_type(inkwell::AddressSpace::default());
        let i32_type = self.context.i32_type();
        let printf_type = i32_type.fn_type(&[i8_ptr_type.into()], true);
        self.module.add_function("printf", printf_type, None)
    }

    pub fn write_object_file(&self, output_path: &str) -> Result<(), String> {
        Target::initialize_native(&InitializationConfig::default())
            .map_err(|e| e.to_string())?;

        let triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&triple)
            .map_err(|e| e.to_string())?;

        let machine = target
            .create_target_machine(
                &triple,
                "generic",
                "",
                OptimizationLevel::None,
                RelocMode::Default,
                CodeModel::Default,
            )
            .ok_or("Failed to create target machine")?;

        machine
            .write_to_file(&self.module, FileType::Object, std::path::Path::new(output_path))
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn print_ir(&self) {
        self.module.print_to_stderr();
    }

    pub fn verify(&self) -> Result<(), String> {
        self.module.verify().map_err(|e| e.to_string())
    }
}