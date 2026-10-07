#[doc = "Register `XTALSTDSR` reader"]
pub type R = crate::R<XtalstdsrSpec>;
#[doc = "Register `XTALSTDSR` writer"]
pub type W = crate::W<XtalstdsrSpec>;
#[doc = "Field `XTALSTDF` reader - desc XTALSTDF"]
pub type XtalstdfR = crate::BitReader;
#[doc = "Field `XTALSTDF` writer - desc XTALSTDF"]
pub type XtalstdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XTALSTDF"]
    #[inline(always)]
    pub fn xtalstdf(&self) -> XtalstdfR {
        XtalstdfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XTALSTDF"]
    #[inline(always)]
    pub fn xtalstdf(&mut self) -> XtalstdfW<'_, XtalstdsrSpec> {
        XtalstdfW::new(self, 0)
    }
}
#[doc = "desc XTALSTDSR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalstdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalstdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalstdsrSpec;
impl crate::RegisterSpec for XtalstdsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtalstdsr::R`](R) reader structure"]
impl crate::Readable for XtalstdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalstdsr::W`](W) writer structure"]
impl crate::Writable for XtalstdsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALSTDSR to value 0"]
impl crate::Resettable for XtalstdsrSpec {}
