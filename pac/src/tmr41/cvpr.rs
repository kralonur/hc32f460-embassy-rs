#[doc = "Register `CVPR` reader"]
pub type R = crate::R<CvprSpec>;
#[doc = "Register `CVPR` writer"]
pub type W = crate::W<CvprSpec>;
#[doc = "Field `ZIM` reader - desc ZIM"]
pub type ZimR = crate::FieldReader;
#[doc = "Field `ZIM` writer - desc ZIM"]
pub type ZimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PIM` reader - desc PIM"]
pub type PimR = crate::FieldReader;
#[doc = "Field `PIM` writer - desc PIM"]
pub type PimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ZIC` reader - desc ZIC"]
pub type ZicR = crate::FieldReader;
#[doc = "Field `PIC` reader - desc PIC"]
pub type PicR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - desc ZIM"]
    #[inline(always)]
    pub fn zim(&self) -> ZimR {
        ZimR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - desc PIM"]
    #[inline(always)]
    pub fn pim(&self) -> PimR {
        PimR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - desc ZIC"]
    #[inline(always)]
    pub fn zic(&self) -> ZicR {
        ZicR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - desc PIC"]
    #[inline(always)]
    pub fn pic(&self) -> PicR {
        PicR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc ZIM"]
    #[inline(always)]
    pub fn zim(&mut self) -> ZimW<'_, CvprSpec> {
        ZimW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc PIM"]
    #[inline(always)]
    pub fn pim(&mut self) -> PimW<'_, CvprSpec> {
        PimW::new(self, 4)
    }
}
#[doc = "desc CVPR\n\nYou can [`read`](crate::Reg::read) this register and get [`cvpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cvpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvprSpec;
impl crate::RegisterSpec for CvprSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cvpr::R`](R) reader structure"]
impl crate::Readable for CvprSpec {}
#[doc = "`write(|w| ..)` method takes [`cvpr::W`](W) writer structure"]
impl crate::Writable for CvprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CVPR to value 0"]
impl crate::Resettable for CvprSpec {}
