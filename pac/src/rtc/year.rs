#[doc = "Register `YEAR` reader"]
pub type R = crate::R<YearSpec>;
#[doc = "Register `YEAR` writer"]
pub type W = crate::W<YearSpec>;
#[doc = "Field `YEARU` reader - desc YEARU"]
pub type YearuR = crate::FieldReader;
#[doc = "Field `YEARU` writer - desc YEARU"]
pub type YearuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEARD` reader - desc YEARD"]
pub type YeardR = crate::FieldReader;
#[doc = "Field `YEARD` writer - desc YEARD"]
pub type YeardW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - desc YEARU"]
    #[inline(always)]
    pub fn yearu(&self) -> YearuR {
        YearuR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - desc YEARD"]
    #[inline(always)]
    pub fn yeard(&self) -> YeardR {
        YeardR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc YEARU"]
    #[inline(always)]
    pub fn yearu(&mut self) -> YearuW<'_, YearSpec> {
        YearuW::new(self, 0)
    }
    #[doc = "Bits 4:7 - desc YEARD"]
    #[inline(always)]
    pub fn yeard(&mut self) -> YeardW<'_, YearSpec> {
        YeardW::new(self, 4)
    }
}
#[doc = "desc YEAR\n\nYou can [`read`](crate::Reg::read) this register and get [`year::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`year::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct YearSpec;
impl crate::RegisterSpec for YearSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`year::R`](R) reader structure"]
impl crate::Readable for YearSpec {}
#[doc = "`write(|w| ..)` method takes [`year::W`](W) writer structure"]
impl crate::Writable for YearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets YEAR to value 0"]
impl crate::Resettable for YearSpec {}
