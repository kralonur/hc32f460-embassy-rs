#[doc = "Register `FEA` writer"]
pub type W = crate::W<FeaSpec>;
#[doc = "Field `FNE` writer - desc FNE"]
pub type FneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTOE` writer - desc FTOE"]
pub type FtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCE` writer - desc FCE"]
pub type FceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEBE` writer - desc FEBE"]
pub type FebeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIE` writer - desc FIE"]
pub type FieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCMDE` writer - desc FCMDE"]
pub type FcmdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc FNE"]
    #[inline(always)]
    pub fn fne(&mut self) -> FneW<'_, FeaSpec> {
        FneW::new(self, 0)
    }
    #[doc = "Bit 1 - desc FTOE"]
    #[inline(always)]
    pub fn ftoe(&mut self) -> FtoeW<'_, FeaSpec> {
        FtoeW::new(self, 1)
    }
    #[doc = "Bit 2 - desc FCE"]
    #[inline(always)]
    pub fn fce(&mut self) -> FceW<'_, FeaSpec> {
        FceW::new(self, 2)
    }
    #[doc = "Bit 3 - desc FEBE"]
    #[inline(always)]
    pub fn febe(&mut self) -> FebeW<'_, FeaSpec> {
        FebeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc FIE"]
    #[inline(always)]
    pub fn fie(&mut self) -> FieW<'_, FeaSpec> {
        FieW::new(self, 4)
    }
    #[doc = "Bit 7 - desc FCMDE"]
    #[inline(always)]
    pub fn fcmde(&mut self) -> FcmdeW<'_, FeaSpec> {
        FcmdeW::new(self, 7)
    }
}
#[doc = "desc FEA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fea::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeaSpec;
impl crate::RegisterSpec for FeaSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`fea::W`](W) writer structure"]
impl crate::Writable for FeaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FEA to value 0"]
impl crate::Resettable for FeaSpec {}
