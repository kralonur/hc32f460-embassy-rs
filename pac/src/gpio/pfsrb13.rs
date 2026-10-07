#[doc = "Register `PFSRB13` reader"]
pub type R = crate::R<Pfsrb13Spec>;
#[doc = "Register `PFSRB13` writer"]
pub type W = crate::W<Pfsrb13Spec>;
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
    pub fn fsel(&mut self) -> FselW<'_, Pfsrb13Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsrb13Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRB13\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsrb13Spec;
impl crate::RegisterSpec for Pfsrb13Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrb13::R`](R) reader structure"]
impl crate::Readable for Pfsrb13Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsrb13::W`](W) writer structure"]
impl crate::Writable for Pfsrb13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRB13 to value 0"]
impl crate::Resettable for Pfsrb13Spec {}
