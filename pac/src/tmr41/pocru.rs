#[doc = "Register `POCRU` reader"]
pub type R = crate::R<PocruSpec>;
#[doc = "Register `POCRU` writer"]
pub type W = crate::W<PocruSpec>;
#[doc = "Field `DIVCK` reader - desc DIVCK"]
pub type DivckR = crate::FieldReader;
#[doc = "Field `DIVCK` writer - desc DIVCK"]
pub type DivckW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PWMMD` reader - desc PWMMD"]
pub type PwmmdR = crate::FieldReader;
#[doc = "Field `PWMMD` writer - desc PWMMD"]
pub type PwmmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LVLS` reader - desc LVLS"]
pub type LvlsR = crate::FieldReader;
#[doc = "Field `LVLS` writer - desc LVLS"]
pub type LvlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - desc DIVCK"]
    #[inline(always)]
    pub fn divck(&self) -> DivckR {
        DivckR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - desc PWMMD"]
    #[inline(always)]
    pub fn pwmmd(&self) -> PwmmdR {
        PwmmdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - desc LVLS"]
    #[inline(always)]
    pub fn lvls(&self) -> LvlsR {
        LvlsR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - desc DIVCK"]
    #[inline(always)]
    pub fn divck(&mut self) -> DivckW<'_, PocruSpec> {
        DivckW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc PWMMD"]
    #[inline(always)]
    pub fn pwmmd(&mut self) -> PwmmdW<'_, PocruSpec> {
        PwmmdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc LVLS"]
    #[inline(always)]
    pub fn lvls(&mut self) -> LvlsW<'_, PocruSpec> {
        LvlsW::new(self, 6)
    }
}
#[doc = "desc POCRU\n\nYou can [`read`](crate::Reg::read) this register and get [`pocru::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocru::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PocruSpec;
impl crate::RegisterSpec for PocruSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pocru::R`](R) reader structure"]
impl crate::Readable for PocruSpec {}
#[doc = "`write(|w| ..)` method takes [`pocru::W`](W) writer structure"]
impl crate::Writable for PocruSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POCRU to value 0xff00"]
impl crate::Resettable for PocruSpec {
    const RESET_VALUE: u16 = 0xff00;
}
