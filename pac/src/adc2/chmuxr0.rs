#[doc = "Register `CHMUXR0` reader"]
pub type R = crate::R<Chmuxr0Spec>;
#[doc = "Register `CHMUXR0` writer"]
pub type W = crate::W<Chmuxr0Spec>;
#[doc = "Field `CH00MUX` reader - desc CH00MUX"]
pub type Ch00muxR = crate::FieldReader;
#[doc = "Field `CH00MUX` writer - desc CH00MUX"]
pub type Ch00muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH01MUX` reader - desc CH01MUX"]
pub type Ch01muxR = crate::FieldReader;
#[doc = "Field `CH01MUX` writer - desc CH01MUX"]
pub type Ch01muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH02MUX` reader - desc CH02MUX"]
pub type Ch02muxR = crate::FieldReader;
#[doc = "Field `CH02MUX` writer - desc CH02MUX"]
pub type Ch02muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH03MUX` reader - desc CH03MUX"]
pub type Ch03muxR = crate::FieldReader;
#[doc = "Field `CH03MUX` writer - desc CH03MUX"]
pub type Ch03muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CH00MUX"]
    #[inline(always)]
    pub fn ch00mux(&self) -> Ch00muxR {
        Ch00muxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CH01MUX"]
    #[inline(always)]
    pub fn ch01mux(&self) -> Ch01muxR {
        Ch01muxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CH02MUX"]
    #[inline(always)]
    pub fn ch02mux(&self) -> Ch02muxR {
        Ch02muxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CH03MUX"]
    #[inline(always)]
    pub fn ch03mux(&self) -> Ch03muxR {
        Ch03muxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CH00MUX"]
    #[inline(always)]
    pub fn ch00mux(&mut self) -> Ch00muxW<'_, Chmuxr0Spec> {
        Ch00muxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CH01MUX"]
    #[inline(always)]
    pub fn ch01mux(&mut self) -> Ch01muxW<'_, Chmuxr0Spec> {
        Ch01muxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc CH02MUX"]
    #[inline(always)]
    pub fn ch02mux(&mut self) -> Ch02muxW<'_, Chmuxr0Spec> {
        Ch02muxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc CH03MUX"]
    #[inline(always)]
    pub fn ch03mux(&mut self) -> Ch03muxW<'_, Chmuxr0Spec> {
        Ch03muxW::new(self, 12)
    }
}
#[doc = "desc CHMUXR0\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmuxr0Spec;
impl crate::RegisterSpec for Chmuxr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chmuxr0::R`](R) reader structure"]
impl crate::Readable for Chmuxr0Spec {}
#[doc = "`write(|w| ..)` method takes [`chmuxr0::W`](W) writer structure"]
impl crate::Writable for Chmuxr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHMUXR0 to value 0x3210"]
impl crate::Resettable for Chmuxr0Spec {
    const RESET_VALUE: u16 = 0x3210;
}
