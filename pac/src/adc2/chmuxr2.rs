#[doc = "Register `CHMUXR2` reader"]
pub type R = crate::R<Chmuxr2Spec>;
#[doc = "Register `CHMUXR2` writer"]
pub type W = crate::W<Chmuxr2Spec>;
#[doc = "Field `CH08MUX` reader - desc CH08MUX"]
pub type Ch08muxR = crate::FieldReader;
#[doc = "Field `CH08MUX` writer - desc CH08MUX"]
pub type Ch08muxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc CH08MUX"]
    #[inline(always)]
    pub fn ch08mux(&self) -> Ch08muxR {
        Ch08muxR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc CH08MUX"]
    #[inline(always)]
    pub fn ch08mux(&mut self) -> Ch08muxW<'_, Chmuxr2Spec> {
        Ch08muxW::new(self, 0)
    }
}
#[doc = "desc CHMUXR2\n\nYou can [`read`](crate::Reg::read) this register and get [`chmuxr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chmuxr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Chmuxr2Spec;
impl crate::RegisterSpec for Chmuxr2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`chmuxr2::R`](R) reader structure"]
impl crate::Readable for Chmuxr2Spec {}
#[doc = "`write(|w| ..)` method takes [`chmuxr2::W`](W) writer structure"]
impl crate::Writable for Chmuxr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHMUXR2 to value 0xba98"]
impl crate::Resettable for Chmuxr2Spec {
    const RESET_VALUE: u16 = 0xba98;
}
