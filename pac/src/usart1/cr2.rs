#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `MPE` reader - desc MPE"]
pub type MpeR = crate::BitReader;
#[doc = "Field `MPE` writer - desc MPE"]
pub type MpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKC` reader - desc CLKC"]
pub type ClkcR = crate::FieldReader;
#[doc = "Field `CLKC` writer - desc CLKC"]
pub type ClkcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STOP` reader - desc STOP"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - desc STOP"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc MPE"]
    #[inline(always)]
    pub fn mpe(&self) -> MpeR {
        MpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 11:12 - desc CLKC"]
    #[inline(always)]
    pub fn clkc(&self) -> ClkcR {
        ClkcR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - desc STOP"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc MPE"]
    #[inline(always)]
    pub fn mpe(&mut self) -> MpeW<'_, Cr2Spec> {
        MpeW::new(self, 0)
    }
    #[doc = "Bits 11:12 - desc CLKC"]
    #[inline(always)]
    pub fn clkc(&mut self) -> ClkcW<'_, Cr2Spec> {
        ClkcW::new(self, 11)
    }
    #[doc = "Bit 13 - desc STOP"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, Cr2Spec> {
        StopW::new(self, 13)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
