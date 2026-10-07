#[doc = "Register `CHMUXR1` reader"]
pub type R = crate::R<Chmuxr1Spec>;
#[doc = "Register `CHMUXR1` writer"]
pub type W = crate::W<Chmuxr1Spec>;
#[doc = "Field `CH04MUX` reader - desc CH04MUX"]
pub type Ch04muxR = crate::FieldReader;
#[doc = "Field `CH04MUX` writer - desc CH04MUX"]
pub type Ch04muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH05MUX` reader - desc CH05MUX"]
pub type Ch05muxR = crate::FieldReader;
#[doc = "Field `CH05MUX` writer - desc CH05MUX"]
pub type Ch05muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH06MUX` reader - desc CH06MUX"]
pub type Ch06muxR = crate::FieldReader;
#[doc = "Field `CH06MUX` writer - desc CH06MUX"]
pub type Ch06muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH07MUX` reader - desc CH07MUX"]
pub type Ch07muxR = crate::FieldReader;
#[doc = "Field `CH07MUX` writer - desc CH07MUX"]
pub type Ch07muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CH04MUX"]
    #[inline(always)]
    pub fn ch04mux(&self) -> Ch04muxR {
        Ch04muxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc CH05MUX"]
    #[inline(always)]
    pub fn ch05mux(&self) -> Ch05muxR {
        Ch05muxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc CH06MUX"]
    #[inline(always)]
    pub fn ch06mux(&self) -> Ch06muxR {
        Ch06muxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc CH07MUX"]
    #[inline(always)]
    pub fn ch07mux(&self) -> Ch07muxR {
        Ch07muxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CH04MUX"]
    #[inline(always)]
    pub fn ch04mux(&mut self) -> Ch04muxW<'_, Chmuxr1Spec> {
        Ch04muxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc CH05MUX"]
    #[inline(always)]
    pub fn ch05mux(&mut self) -> Ch05muxW<'_, Chmuxr1Spec> {
        Ch05muxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - desc CH06MUX"]
    #[inline(always)]
    pub fn ch06mux(&mut self) -> Ch06muxW<'_, Chmuxr1Spec> {
        Ch06muxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - desc CH07MUX"]
    #[inline(always)]
    pub fn ch07mux(&mut self) -> Ch07muxW<'_, Chmuxr1Spec> {
        Ch07muxW::new(self, 12)
    }
}
#[doc = "desc CHMUXR1\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmuxr1Spec;
impl crate::RegisterSpec for Chmuxr1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chmuxr1::R`](R) reader structure"]
impl crate::Readable for Chmuxr1Spec {}
#[doc = "`write(|w| ..)` method takes [`chmuxr1::W`](W) writer structure"]
impl crate::Writable for Chmuxr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHMUXR1 to value 0x7654"]
impl crate::Resettable for Chmuxr1Spec {
    const RESET_VALUE: u16 = 0x7654;
}
