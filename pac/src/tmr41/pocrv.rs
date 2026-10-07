#[doc = "Register `POCRV` reader"]
pub type R = crate::R<PocrvSpec>;
#[doc = "Register `POCRV` writer"]
pub type W = crate::W<PocrvSpec>;
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
    pub fn divck(&mut self) -> DivckW<'_, PocrvSpec> {
        DivckW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc PWMMD"]
    #[inline(always)]
    pub fn pwmmd(&mut self) -> PwmmdW<'_, PocrvSpec> {
        PwmmdW::new(self, 4)
    }
    #[doc = "Bits 6:7 - desc LVLS"]
    #[inline(always)]
    pub fn lvls(&mut self) -> LvlsW<'_, PocrvSpec> {
        LvlsW::new(self, 6)
    }
}
#[doc = "desc POCRV\n\nYou can [`read`](crate::Reg::read) this register and get [`pocrv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pocrv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PocrvSpec;
impl crate::RegisterSpec for PocrvSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pocrv::R`](R) reader structure"]
impl crate::Readable for PocrvSpec {}
#[doc = "`write(|w| ..)` method takes [`pocrv::W`](W) writer structure"]
impl crate::Writable for PocrvSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POCRV to value 0xff00"]
impl crate::Resettable for PocrvSpec {
    const RESET_VALUE: u16 = 0xff00;
}
