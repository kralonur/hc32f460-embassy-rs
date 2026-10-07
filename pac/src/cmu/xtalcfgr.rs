#[doc = "Register `XTALCFGR` reader"]
pub type R = crate::R<XtalcfgrSpec>;
#[doc = "Register `XTALCFGR` writer"]
pub type W = crate::W<XtalcfgrSpec>;
#[doc = "Field `XTALDRV` reader - desc XTALDRV"]
pub type XtaldrvR = crate::FieldReader;
#[doc = "Field `XTALDRV` writer - desc XTALDRV"]
pub type XtaldrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XTALMS` reader - desc XTALMS"]
pub type XtalmsR = crate::BitReader;
#[doc = "Field `XTALMS` writer - desc XTALMS"]
pub type XtalmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUPDRV` reader - desc SUPDRV"]
pub type SupdrvR = crate::BitReader;
#[doc = "Field `SUPDRV` writer - desc SUPDRV"]
pub type SupdrvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - desc XTALDRV"]
    #[inline(always)]
    pub fn xtaldrv(&self) -> XtaldrvR {
        XtaldrvR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - desc XTALMS"]
    #[inline(always)]
    pub fn xtalms(&self) -> XtalmsR {
        XtalmsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc SUPDRV"]
    #[inline(always)]
    pub fn supdrv(&self) -> SupdrvR {
        SupdrvR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - desc XTALDRV"]
    #[inline(always)]
    pub fn xtaldrv(&mut self) -> XtaldrvW<'_, XtalcfgrSpec> {
        XtaldrvW::new(self, 4)
    }
    #[doc = "Bit 6 - desc XTALMS"]
    #[inline(always)]
    pub fn xtalms(&mut self) -> XtalmsW<'_, XtalcfgrSpec> {
        XtalmsW::new(self, 6)
    }
    #[doc = "Bit 7 - desc SUPDRV"]
    #[inline(always)]
    pub fn supdrv(&mut self) -> SupdrvW<'_, XtalcfgrSpec> {
        SupdrvW::new(self, 7)
    }
}
#[doc = "desc XTALCFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XtalcfgrSpec;
impl crate::RegisterSpec for XtalcfgrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`xtalcfgr::R`](R) reader structure"]
impl crate::Readable for XtalcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`xtalcfgr::W`](W) writer structure"]
impl crate::Writable for XtalcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALCFGR to value 0x80"]
impl crate::Resettable for XtalcfgrSpec {
    const RESET_VALUE: u8 = 0x80;
}
