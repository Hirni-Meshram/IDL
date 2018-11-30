use ::flat_ast::*;
use std::io::{Result, Write};
use ::heck::*;

#[macro_use]
pub mod macros {
    #[macro_export]
    macro_rules! cg {
        ($v:expr) => (($v).write("")?);
        ($v:expr, $fmt:expr) => (($v).write(format!($fmt))?);
        ($v:expr, $fmt:expr, $($arg:tt)*) => (($v).write(format!($fmt, $($arg)*))?);
    }
}

pub (super) struct CodeHeaderGenerator<'a, W: Write + 'a> {
    writer: &'a mut ::writer::Writer<W>
}

impl<'a, W: Write> CodeHeaderGenerator<'a, W> {
    pub fn new(writer: &'a mut ::writer::Writer<W>) -> Self {
        Self {
            writer
        }
    }

    fn indent(&mut self) {
        self.writer.indent();
    }

    fn dedent(&mut self) {
        self.writer.dedent();
    }

    fn write(&mut self, val: impl AsRef<str>) -> Result<&mut Self> {
        self.writer.write(val)?;
        Ok(self)
    }

    pub fn generate(&mut self, packet: &Packet) -> Result<()> {
        cg!(self, "#pragma once\n");
        self.doc(packet.doc())?;
        cg!(self);
        cg!(self, r#"#include "packetfactory.h""#);
        
        for content in packet.contents() {
            use self::PacketContent::*;
            match content {
                Include(ref inc, system) => { 
                    if *system {
                        cg!(self, r#"#include <{}>"#, inc);
                    } else {
                        cg!(self, r#"#include "{}""#, inc);
                    }
                },
                _ => {}
            };
        }

        cg!(self);
        cg!(self, r#"namespace RoseCommon {{
namespace Packet {{

REGISTER_RECV_PACKET(ePacketType::{0}, {1})
REGISTER_SEND_PACKET(ePacketType::{0}, {1})"#,
            packet.type_(), packet.class_name());
        
        cg!(self, "class {} : public CRosePacket {{", packet.class_name());
        self.indent();
        cg!(self, "public:");
        self.indent();
        cg!(self, "{}();", packet.class_name());
        cg!(self, "{}(CRoseReader reader);", packet.class_name());
        cg!(self, "{0}({0}&&) = default;", packet.class_name());
        cg!(self, "{0}& operator=({0}&&) = default;", packet.class_name());
        cg!(self, "~{}() = default;", packet.class_name());
        cg!(self);

        for content in packet.contents() {
            use self::PacketContent::*;
            match content {
                Simple(ref simple) => self.simple_type(simple)?,
                _ => {}
            };
        }

        cg!(self);

        for content in packet.contents() {
            use self::PacketContent::*;
            match content {
                Complex(ref complex) => self.complex_type(complex)?,
                _ => {}
            };
        }

        cg!(self);

        for content in packet.contents() {
            use self::PacketContent::*;
            match content {
                Element(ref elem) => {
                    self.elem_setter(elem)?;
                    self.elem_getter(elem)?;
                },
                _ => {}
            };
        }

        cg!(self);
        cg!(self);
        self.create(packet)?;

        self.dedent();
        cg!(self);
        cg!(self, "protected:");
        self.indent();
        cg!(self, "virtual void pack(CRoseBasePolicy&) const override;");
        self.dedent();
        cg!(self);
        cg!(self, "private:");
        self.indent();

        for content in packet.contents() {
            use self::PacketContent::*;
            match content {
                Element(ref elem) => self.element(elem)?,
                _ => {}
            };
        }

        self.dedent();
        self.dedent();
        cg!(self, "}};");
        cg!(self);
        cg!(self, "}}\n}}");
        Ok(())
    }

    fn doc(&mut self, doc: &Option<String>) -> Result<()> {
        match doc {
            None => (),
            Some(doc) => {
                for line in doc.lines() {
                    match line.trim() {
                        "" => (),
                        line => {
                            cg!(self, "// {}", line);
                        }
                    }
                }
            }
        };
        Ok(())
    }

    fn complex_type(&mut self, complex: &ComplexType) -> Result<()> {
        Ok(())
    }

    fn simple_type(&mut self, simple: &SimpleType) -> Result<()> {
        cg!(self);
        self.doc(simple.doc())?;
        for content in simple.contents() {
            match content {
                SimpleTypeContent::Restriction(res) => self.restrict(res, simple.name())?            }
        }
        Ok(())
    }

    fn element(&mut self, elem: &Element) -> Result<()> {
        self.doc(elem.doc())?;

        cg!(self, "{} {};", elem.type_(), elem.name());
        Ok(())
    }

    fn elem_setter(&mut self, elem: &Element) -> Result<()> {
        let reference = if elem.reference() { "&" } else { "" };
        cg!(self, "void set_{}(const {}{});", elem.name(), elem.type_(), reference);
        Ok(())
    }

    fn elem_getter(&mut self, elem: &Element) -> Result<()> {
        let reference = if elem.reference() { "&" } else { "" };
        cg!(self, "const {}{} get_{}() const;", elem.type_(), reference, elem.name());
        Ok(())
    }

    fn create(&mut self, packet: &Packet) -> Result<()> {
        let args = packet.contents().iter().map(|elem| {
            use self::PacketContent::*;
            match elem {
                Element(ref e) => match e.init() {
                        self::ElementInitValue::Create => "const ".to_owned() + e.type_() + "&, ",
                        _ => "".to_owned()
                    },
                _ => "".to_string()
            }
        }).collect::<String>();
        let args = if args.len() != 0 {
            &args[..args.len() - 2]
        } else {
            &args
        };
        cg!(self, "static {} create({});", packet.class_name(), args);
        Ok(())
    }

    fn restrict(&mut self, restrict: &Restriction, name: &str) -> Result<()> {
        use self::RestrictionContent::*;
        let is_enum = restrict.contents().iter().find(|content| match content {
            Enumeration(_) => true,
            _ => false
        }).is_some();
        self.doc(restrict.doc())?;
        let base = restrict.base();

        if is_enum {
            cg!(self, "enum {} : {} {{", name, base);
            self.indent();
            for content in restrict.contents() {
                if let Enumeration(en) = content {
                    self.doc(en.doc())?;
                    cg!(self, "{} = {},", en.value(), en.id());
                }
            }
        } else {
            cg!(self, "struct {} : public ISerialize {{", name);
            self.indent();
            cg!(self, "explicit {}();", name);
            cg!(self, "explicit {}({});", name, base);
            cg!(self, "{0}(const {0}&) = default;", name);
            cg!(self, "{0}({0}&&) = default;", name);
            cg!(self, "{0}& operator=(const {0}&) = default;", name);
            cg!(self, "{0}& operator=({0}&&) = default;", name);
            cg!(self, "virtual ~{}() = default;", name);
            cg!(self);
            cg!(self, "operator {}() const {{ return {}; }}", base, name.to_string().to_snake_case());
            cg!(self, "bool isValid() const {{ return is_valid; }}");
            cg!(self);
            cg!(self, "virtual bool read(CRoseReader&) override;");
            cg!(self, "virtual bool write(CRoseBasePolicy&) const override;");
            cg!(self);
            cg!(self, "private:");
            self.indent();
            cg!(self, "{} {};", base, name.to_string().to_snake_case());
            cg!(self, "bool is_valid;");
            self.dedent();
        }

        self.dedent();
        cg!(self, "}};");
        Ok(())
    }
}