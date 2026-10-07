#[doc = "Register `XTALSTBCR` reader"]
pub type R = crate::R<XtalstbcrSpec>;
#[doc = "Register `XTALSTBCR` writer"]
pub type W = crate::W<XtalstbcrSpec>;
#[doc = "Field `XTALSTB` reader - desc XTALSTB"]
pub type XtalstbR = crate::FieldReader;
#[doc = "Field `XTALSTB` writer - desc XTALSTB"]
pub type XtalstbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc XTALSTB"]
    #[inline(always)]
    pub fn xtalstb(&self) -> XtalstbR {
        XtalstbR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc XTALSTB"]
    #[inline(always)]
    pub fn xtalstb(&mut self) -> XtalstbW<'_, XtalstbcrSpec> {
        XtalstbW::new(self, 0)
    }
}
#[doc = "desc XTALSTBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstbcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstbcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalstbcrSpec;
impl crate::RegisterSpec for XtalstbcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtalstbcr::R`](R) reader structure"]
impl crate::Readable for XtalstbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalstbcr::W`](W) writer structure"]
impl crate::Writable for XtalstbcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALSTBCR to value 0x05"]
impl crate::Resettable for XtalstbcrSpec {
    const RESET_VALUE: u8 = 0x05;
}
