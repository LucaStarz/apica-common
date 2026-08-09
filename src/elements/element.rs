use crate::bytecodes::types::ApicaTypeBytecode;
use crate::elements::modifier::ElementModifier;
use crate::values::value::{Value, ValueTrait};
use crate::values::vtype::get_vtype_repr;

/// Represents a runtime value unit paired with execution metadata modifiers.
///
/// An `Element` wraps an underlying [`Value`] and an [`ElementModifier`] bitflag set,
/// enabling the runtime interpreter to check for type traits, mutability, error states,
/// or active control flow interruptions during evaluation.
pub struct Element {
    /// Bitflags defining attributes and control state associated with this element.
    pub modifier: ElementModifier,

    /// The actual underlying evaluated value.
    value: Value,
}

impl Element {
    /// Creates a new [`Element`] wrapping the given [`ElementModifier`] and [`Value`].
    pub fn new(modifier: ElementModifier, value: Value) -> Element {
        Element { modifier, value }
    }

    /// Returns a shared reference to the inner [`Value`].
    pub fn get_value(&self) -> &Value {
        &self.value
    }

    /// Checks whether this element carries the [`ElementModifier::ERROR`] flag.
    ///
    /// # Returns
    ///
    /// `true` if the elements represents a runtime error, `false` otherwise.
    pub fn is_error(&self) -> bool {
        self.modifier.contains(ElementModifier::ERROR)
    }

    /// Checks whether this element carries an error flag or any active control flow modifier.
    ///
    /// Control modifiers include [`ElementModifier::ERROR`], [`ElementModifier::BREAK`],
    /// [`ElementModifier::CONTINUE`], [`ElementModifier::RETURN`], and [`ElementModifier::TERMINATE`].
    ///
    /// # Returns
    ///
    /// `true` if execution flow needs to be interrupted or redirected, `false` otherwise.
    pub fn is_error_or_control(&self) -> bool {
        self.modifier.intersects(ElementModifier::ERROR | ElementModifier::BREAK | ElementModifier::CONTINUE | ElementModifier::RETURN | ElementModifier::TERMINATE)
    }

    pub fn add(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("+", false),
            );
        }

        let result = self.value.add(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("+", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }

    pub fn increment(&mut self) -> Element {
        if self.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("right ++", true),
            )
        }

        let result = self.value.increment();
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("right ++", self.value.get_type_repr()),
            )
        }
    }

    pub fn left_increment(&mut self) -> Element {
        if self.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("left ++", true),
            )
        }

        let result = self.value.left_increment();
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("left ++", self.value.get_type_repr()),
            )
        }
    }
    
    pub fn subtract(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("-", false),
            );
        }

        let result = self.value.subtract(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("-", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn decrement(&mut self) -> Element {
        if self.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("right --", true),
            )
        }

        let result = self.value.decrement();
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("right --", self.value.get_type_repr()),
            )
        }
    }
    
    pub fn left_decrement(&mut self) -> Element {
        if self.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("left --", true),
            )
        }

        let result = self.value.left_decrement();
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("left --", self.value.get_type_repr()),
            )
        }
    }
    
    pub fn times(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("*", false),
            );
        }

        let result = self.value.times(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("*", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn unary_not(&self) -> Element {
        let result = self.value.unary_not();
        match result { 
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("!", self.value.get_type_repr()),
            )
        }
    }
    
    pub fn bitwise_not(&self) -> Element {
        let result = self.value.bitwise_not();
        match result { 
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::unary_operation_error("~", self.value.get_type_repr()),
            )
        }
    }
    
    pub fn less_than(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("<", false),
            );
        }

        let result = self.value.less_than(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("<", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn less_or_equal(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error("<=", false),
            );
        }

        let result = self.value.less_or_equal(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("<=", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn greater_than(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error(">", false),
            );
        }

        let result = self.value.greater_than(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error(">", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn greater_or_equal(&self, other: &Element) -> Element {
        if self.value.is_null() || other.value.is_null() {
            return Element::new(
                ElementModifier::ERROR,
                Value::null_operation_error(">=", false),
            );
        }

        let result = self.value.greater_or_equal(&other.value);
        match result {
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error(">=", self.value.get_type_repr(), other.value.get_type_repr()),
            ),
        }
    }
    
    pub fn convert(&self, to: ApicaTypeBytecode) -> Element {
        let auto_converted = self.value.auto_convert(to);
        if let Some(auto) = auto_converted {
            return Element::new(ElementModifier::NONE, auto);
        }
        
        let converted = self.value.convert(to);
        match converted { 
            Some(val) => Element::new(ElementModifier::NONE, val),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("as", self.value.get_type_repr(), get_vtype_repr(to)),
            )
        }
    }
    
    pub fn auto_convert(&self, to: ApicaTypeBytecode) -> Element {
        let auto_converted = self.value.auto_convert(to);
        match auto_converted { 
            Some(auto) => Element::new(ElementModifier::NONE, auto),
            None => Element::new(
                ElementModifier::ERROR,
                Value::binary_operation_error("auto-as", self.value.get_type_repr(), get_vtype_repr(to)),
            )
        }
    }
    
    pub fn check_and_convert(&mut self, to: ApicaTypeBytecode) {
        if self.is_error_or_control() {
            return;
        }
        
        if to == ApicaTypeBytecode::Any {
            self.modifier |= ElementModifier::ANY;
            return;
        }
        
        let auto_converted = self.value.auto_convert(to);
        if let Some(auto) = auto_converted {
            self.value = auto;
            return;
        }
        
        let converted = self.value.convert(to);
        match converted { 
            Some(conv) => self.value = conv,
            None => {
                self.value = Value::binary_operation_error(
                    "as", self.value.get_type_repr(), get_vtype_repr(to)
                );
                self.modifier |= ElementModifier::ERROR;
            }
        }
    }
}