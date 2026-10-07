#[doc = "Register `PFSRB3` reader"]
pub type R = crate::R<Pfsrb3Spec>;
#[doc = "Register `PFSRB3` writer"]
pub type W = crate::W<Pfsrb3Spec>;
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
    pub fn fsel(&mut self) -> FselW<'_, Pfsrb3Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsrb3Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRB3\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrb3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrb3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsrb3Spec;
impl crate::RegisterSpec for Pfsrb3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrb3::R`](R) reader structure"]
impl crate::Readable for Pfsrb3Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsrb3::W`](W) writer structure"]
impl crate::Writable for Pfsrb3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRB3 to value 0"]
impl crate::Resettable for Pfsrb3Spec {}
