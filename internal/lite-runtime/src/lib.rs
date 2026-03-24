pub mod externalizable;
pub mod global_script_object;
pub mod global_script_object_residence;
pub mod reflect_base_macro;
pub mod resource_registry;
pub mod script_context;
pub mod script_failure;
pub mod script_metadata;
pub mod script_object;
pub mod script_object_fyrox_traits;
pub mod script_object_residence;
pub mod spi;

#[macro_export]
macro_rules! reflect_base_lite {
    () => {
        // fn query_derived_types(&self) -> &'static [core::any::TypeId] {
        //     Self::derived_types()
        // }

        // fn derived_types() -> &'static [core::any::TypeId] {
        //     &[]
        // }

        // fn try_clone_box(&self) -> Option<Box<dyn Reflect>> {
        //     Some(Box::new(self.clone()))
        // }
    };
}
#[macro_export]
macro_rules! wrapper_reflect {
    ($ident:tt) => {
        fn source_path() -> &'static str
        where
            Self: Sized,
        {
            file!()
        }

        fn assembly_name(&self) -> &'static str {
            env!("CARGO_PKG_NAME")
        }

        fn type_assembly_name() -> &'static str
        where
            Self: Sized,
        {
            env!("CARGO_PKG_NAME")
        }

        fn type_name(&self) -> &'static str {
            Reflect::type_name(&self.$ident)
        }

        fn doc(&self) -> &'static str {
            self.$ident.doc()
        }

        fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
            self.$ident.fields_info(func);
        }

        fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
            self.$ident.fields(func);
        }

        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
            self.$ident.fields_mut(func);
        }

        fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
            fyrox::core::reflect::Reflect::into_any(self.$ident.into())
        }

        fn as_any(&self, func: &mut dyn FnMut(&dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any(&self.$ident, func)
        }

        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any_mut(&mut self.$ident, func)
        }

        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            self.$ident.as_reflect(func)
        }

        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            self.$ident.as_reflect_mut(func)
        }

        fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            self.$ident.set(value)
        }

        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            self.$ident.field(name, func)
        }

        fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
            self.$ident.field_mut(name, func)
        }

        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            self.$ident.as_array(func)
        }

        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            self.$ident.as_array_mut(func)
        }

        fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
            self.$ident.as_list(func)
        }

        fn as_list_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectList>)) {
            self.$ident.as_list_mut(func)
        }

        fn as_inheritable_variable(
            &self,
            func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable(func)
        }

        fn as_inheritable_variable_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable_mut(func)
        }

        fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
            self.$ident.as_hash_map(func)
        }

        fn as_hash_map_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>)) {
            self.$ident.as_hash_map_mut(func)
        }

        // fn query_derived_types(&self) -> &'static [core::any::TypeId] {
        //     self.$ident.query_derived_types()
        // }

        // fn derived_types() -> &'static [core::any::TypeId] {
        //     &[]
        // }

        // fn try_clone_box(&self) -> Option<Box<dyn Reflect>> {
        //     Some(Box::new(self.clone()))
        // }
    };
}
