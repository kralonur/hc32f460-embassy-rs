#[doc = "Register `XCMD` reader"]
pub type R = crate::R<XcmdSpec>;
#[doc = "Register `XCMD` writer"]
pub type W = crate::W<XcmdSpec>;
#[doc = "Field `XIPMC` reader - desc XIPMC"]
pub type XipmcR = crate::FieldReader;
#[doc = "Field `XIPMC` writer - desc XIPMC"]
pub type XipmcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc XIPMC"]
    #[inline(always)]
    pub fn xipmc(&self) -> XipmcR {
        XipmcR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc XIPMC"]
    #[inline(always)]
    pub fn xipmc(&mut self) -> XipmcW<'_, XcmdSpec> {
        XipmcW::new(self, 0)
    }
}
#[doc = "desc XCMD\n\nYou can [`read`](crate::Reg::read) this register and get [`xcmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XcmdSpec;
impl crate::RegisterSpec for XcmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcmd::R`](R) reader structure"]
impl crate::Readable for XcmdSpec {}
#[doc = "`write(|w| ..)` method takes [`xcmd::W`](W) writer structure"]
impl crate::Writable for XcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCMD to value 0xff"]
impl crate::Resettable for XcmdSpec {
    const RESET_VALUE: u32 = 0xff;
}
