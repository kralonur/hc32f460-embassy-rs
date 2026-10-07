#[doc = "Register `HOUR` reader"]
pub type R = crate::R<HourSpec>;
#[doc = "Register `HOUR` writer"]
pub type W = crate::W<HourSpec>;
#[doc = "Field `HOURU` reader - desc HOURU"]
pub type HouruR = crate::FieldReader;
#[doc = "Field `HOURU` writer - desc HOURU"]
pub type HouruW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURD` reader - desc HOURD"]
pub type HourdR = crate::FieldReader;
#[doc = "Field `HOURD` writer - desc HOURD"]
pub type HourdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - desc HOURU"]
    #[inline(always)]
    pub fn houru(&self) -> HouruR {
        HouruR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - desc HOURD"]
    #[inline(always)]
    pub fn hourd(&self) -> HourdR {
        HourdR::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc HOURU"]
    #[inline(always)]
    pub fn houru(&mut self) -> HouruW<'_, HourSpec> {
        HouruW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc HOURD"]
    #[inline(always)]
    pub fn hourd(&mut self) -> HourdW<'_, HourSpec> {
        HourdW::new(self, 4)
    }
}
#[doc = "desc HOUR\n\nYou can [`read`](crate::Reg::read) this register and get [`hour::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hour::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HourSpec;
impl crate::RegisterSpec for HourSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hour::R`](R) reader structure"]
impl crate::Readable for HourSpec {}
#[doc = "`write(|w| ..)` method takes [`hour::W`](W) writer structure"]
impl crate::Writable for HourSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOUR to value 0x12"]
impl crate::Resettable for HourSpec {
    const RESET_VALUE: u8 = 0x12;
}
