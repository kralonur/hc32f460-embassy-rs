#[doc = "Register `XTALCR` reader"]
pub type R = crate::R<XtalcrSpec>;
#[doc = "Register `XTALCR` writer"]
pub type W = crate::W<XtalcrSpec>;
#[doc = "Field `XTALSTP` reader - desc XTALSTP"]
pub type XtalstpR = crate::BitReader;
#[doc = "Field `XTALSTP` writer - desc XTALSTP"]
pub type XtalstpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XTALSTP"]
    #[inline(always)]
    pub fn xtalstp(&self) -> XtalstpR {
        XtalstpR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XTALSTP"]
    #[inline(always)]
    pub fn xtalstp(&mut self) -> XtalstpW<'_, XtalcrSpec> {
        XtalstpW::new(self, 0)
    }
}
#[doc = "desc XTALCR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalcrSpec;
impl crate::RegisterSpec for XtalcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtalcr::R`](R) reader structure"]
impl crate::Readable for XtalcrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalcr::W`](W) writer structure"]
impl crate::Writable for XtalcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALCR to value 0x01"]
impl crate::Resettable for XtalcrSpec {
    const RESET_VALUE: u8 = 0x01;
}
