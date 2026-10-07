#[doc = "Register `ICG0` reader"]
pub type R = crate::R<Icg0Spec>;
#[doc = "Field `SWDTAUTS` reader - desc SWDTAUTS"]
pub type SwdtautsR = crate::BitReader;
#[doc = "Field `SWDTITS` reader - desc SWDTITS"]
pub type SwdtitsR = crate::BitReader;
#[doc = "Field `SWDTPERI` reader - desc SWDTPERI"]
pub type SwdtperiR = crate::FieldReader;
#[doc = "Field `SWDTCKS` reader - desc SWDTCKS"]
pub type SwdtcksR = crate::FieldReader;
#[doc = "Field `SWDTWDPT` reader - desc SWDTWDPT"]
pub type SwdtwdptR = crate::FieldReader;
#[doc = "Field `SWDTSLPOFF` reader - desc SWDTSLPOFF"]
pub type SwdtslpoffR = crate::BitReader;
#[doc = "Field `WDTAUTS` reader - desc WDTAUTS"]
pub type WdtautsR = crate::BitReader;
#[doc = "Field `WDTITS` reader - desc WDTITS"]
pub type WdtitsR = crate::BitReader;
#[doc = "Field `WDTPERI` reader - desc WDTPERI"]
pub type WdtperiR = crate::FieldReader;
#[doc = "Field `WDTCKS` reader - desc WDTCKS"]
pub type WdtcksR = crate::FieldReader;
#[doc = "Field `WDTWDPT` reader - desc WDTWDPT"]
pub type WdtwdptR = crate::FieldReader;
#[doc = "Field `WDTSLPOFF` reader - desc WDTSLPOFF"]
pub type WdtslpoffR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - desc SWDTAUTS"]
    #[inline(always)]
    pub fn swdtauts(&self) -> SwdtautsR {
        SwdtautsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SWDTITS"]
    #[inline(always)]
    pub fn swdtits(&self) -> SwdtitsR {
        SwdtitsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - desc SWDTPERI"]
    #[inline(always)]
    pub fn swdtperi(&self) -> SwdtperiR {
        SwdtperiR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - desc SWDTCKS"]
    #[inline(always)]
    pub fn swdtcks(&self) -> SwdtcksR {
        SwdtcksR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc SWDTWDPT"]
    #[inline(always)]
    pub fn swdtwdpt(&self) -> SwdtwdptR {
        SwdtwdptR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - desc SWDTSLPOFF"]
    #[inline(always)]
    pub fn swdtslpoff(&self) -> SwdtslpoffR {
        SwdtslpoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - desc WDTAUTS"]
    #[inline(always)]
    pub fn wdtauts(&self) -> WdtautsR {
        WdtautsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc WDTITS"]
    #[inline(always)]
    pub fn wdtits(&self) -> WdtitsR {
        WdtitsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc WDTPERI"]
    #[inline(always)]
    pub fn wdtperi(&self) -> WdtperiR {
        WdtperiR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - desc WDTCKS"]
    #[inline(always)]
    pub fn wdtcks(&self) -> WdtcksR {
        WdtcksR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - desc WDTWDPT"]
    #[inline(always)]
    pub fn wdtwdpt(&self) -> WdtwdptR {
        WdtwdptR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - desc WDTSLPOFF"]
    #[inline(always)]
    pub fn wdtslpoff(&self) -> WdtslpoffR {
        WdtslpoffR::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "desc ICG0\n\nYou can [`read`](crate::Reg::read) this register and get [`icg0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Icg0Spec;
impl crate::RegisterSpec for Icg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icg0::R`](R) reader structure"]
impl crate::Readable for Icg0Spec {}
#[doc = "`reset()` method sets ICG0 to value 0xffff_ffff"]
impl crate::Resettable for Icg0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
