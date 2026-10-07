#[doc = "Register `XTAL32CR` reader"]
pub type R = crate::R<Xtal32crSpec>;
#[doc = "Register `XTAL32CR` writer"]
pub type W = crate::W<Xtal32crSpec>;
#[doc = "Field `XTAL32STP` reader - desc XTAL32STP"]
pub type Xtal32stpR = crate::BitReader;
#[doc = "Field `XTAL32STP` writer - desc XTAL32STP"]
pub type Xtal32stpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XTAL32STP"]
    #[inline(always)]
    pub fn xtal32stp(&self) -> Xtal32stpR {
        Xtal32stpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XTAL32STP"]
    #[inline(always)]
    pub fn xtal32stp(&mut self) -> Xtal32stpW<'_, Xtal32crSpec> {
        Xtal32stpW::new(self, 0)
    }
}
#[doc = "desc XTAL32CR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32crSpec;
impl crate::RegisterSpec for Xtal32crSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtal32cr::R`](R) reader structure"]
impl crate::Readable for Xtal32crSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32cr::W`](W) writer structure"]
impl crate::Writable for Xtal32crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32CR to value 0"]
impl crate::Resettable for Xtal32crSpec {}
