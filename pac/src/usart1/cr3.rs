#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `SCEN` reader - desc SCEN"]
pub type ScenR = crate::BitReader;
#[doc = "Field `SCEN` writer - desc SCEN"]
pub type ScenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - desc CTSE"]
pub type CtseR = crate::BitReader;
#[doc = "Field `CTSE` writer - desc CTSE"]
pub type CtseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCN` reader - desc BCN"]
pub type BcnR = crate::FieldReader;
#[doc = "Field `BCN` writer - desc BCN"]
pub type BcnW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 5 - desc SCEN"]
    #[inline(always)]
    pub fn scen(&self) -> ScenR {
        ScenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - desc CTSE"]
    #[inline(always)]
    pub fn ctse(&self) -> CtseR {
        CtseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 21:23 - desc BCN"]
    #[inline(always)]
    pub fn bcn(&self) -> BcnR {
        BcnR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - desc SCEN"]
    #[inline(always)]
    pub fn scen(&mut self) -> ScenW<'_, Cr3Spec> {
        ScenW::new(self, 5)
    }
    #[doc = "Bit 9 - desc CTSE"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CtseW<'_, Cr3Spec> {
        CtseW::new(self, 9)
    }
    #[doc = "Bits 21:23 - desc BCN"]
    #[inline(always)]
    pub fn bcn(&mut self) -> BcnW<'_, Cr3Spec> {
        BcnW::new(self, 21)
    }
}
#[doc = "desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
