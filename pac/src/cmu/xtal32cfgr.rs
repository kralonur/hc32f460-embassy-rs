#[doc = "Register `XTAL32CFGR` reader"]
pub type R = crate::R<Xtal32cfgrSpec>;
#[doc = "Register `XTAL32CFGR` writer"]
pub type W = crate::W<Xtal32cfgrSpec>;
#[doc = "Field `XTAL32DRV` reader - desc XTAL32DRV"]
pub type Xtal32drvR = crate::FieldReader;
#[doc = "Field `XTAL32DRV` writer - desc XTAL32DRV"]
pub type Xtal32drvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - desc XTAL32DRV"]
    #[inline(always)]
    pub fn xtal32drv(&self) -> Xtal32drvR {
        Xtal32drvR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc XTAL32DRV"]
    #[inline(always)]
    pub fn xtal32drv(&mut self) -> Xtal32drvW<'_, Xtal32cfgrSpec> {
        Xtal32drvW::new(self, 0)
    }
}
#[doc = "desc XTAL32CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtal32cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtal32cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtal32cfgrSpec;
impl crate::RegisterSpec for Xtal32cfgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtal32cfgr::R`](R) reader structure"]
impl crate::Readable for Xtal32cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtal32cfgr::W`](W) writer structure"]
impl crate::Writable for Xtal32cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTAL32CFGR to value 0"]
impl crate::Resettable for Xtal32cfgrSpec {}
