#[doc = "Register `PFSRB6` reader"]
pub type R = crate::R<Pfsrb6Spec>;
#[doc = "Register `PFSRB6` writer"]
pub type W = crate::W<Pfsrb6Spec>;
#[doc = "Field `FSEL` reader - desc FSEL"]
pub type FselR = crate::FieldReader;
#[doc = "Field `FSEL` writer - desc FSEL"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BFE` reader - desc BFE"]
pub type BfeR = crate::BitReader;
#[doc = "Field `BFE` writer - desc BFE"]
pub type BfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&self) -> BfeR {
        BfeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc FSEL"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<'_, Pfsrb6Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsrb6Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRB6\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsrb6Spec;
impl crate::RegisterSpec for Pfsrb6Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrb6::R`](R) reader structure"]
impl crate::Readable for Pfsrb6Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsrb6::W`](W) writer structure"]
impl crate::Writable for Pfsrb6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRB6 to value 0"]
impl crate::Resettable for Pfsrb6Spec {}
