#[doc = "Register `DAY` reader"]
pub type R = crate::R<DaySpec>;
#[doc = "Register `DAY` writer"]
pub type W = crate::W<DaySpec>;
#[doc = "Field `DAYU` reader - desc DAYU"]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - desc DAYU"]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYD` reader - desc DAYD"]
pub type DaydR = crate::FieldReader;
#[doc = "Field `DAYD` writer - desc DAYD"]
pub type DaydW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc DAYU"]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - desc DAYD"]
    #[inline(always)]
    pub fn dayd(&self) -> DaydR {
        DaydR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc DAYU"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DayuW<'_, DaySpec> {
        DayuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc DAYD"]
    #[inline(always)]
    pub fn dayd(&mut self) -> DaydW<'_, DaySpec> {
        DaydW::new(self, 4)
    }
}
#[doc = "desc DAY\n\nYou can [`read`](crate::Reg::read) this register and get [`day::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`day::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaySpec;
impl crate::RegisterSpec for DaySpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`day::R`](R) reader structure"]
impl crate::Readable for DaySpec {}
#[doc = "`write(|w| ..)` method takes [`day::W`](W) writer structure"]
impl crate::Writable for DaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAY to value 0"]
impl crate::Resettable for DaySpec {}
