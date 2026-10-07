#[doc = "Register `PFSRA9` reader"]
pub type R = crate::R<Pfsra9Spec>;
#[doc = "Register `PFSRA9` writer"]
pub type W = crate::W<Pfsra9Spec>;
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
    pub fn fsel(&mut self) -> FselW<'_, Pfsra9Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsra9Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRA9\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsra9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsra9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsra9Spec;
impl crate::RegisterSpec for Pfsra9Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsra9::R`](R) reader structure"]
impl crate::Readable for Pfsra9Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsra9::W`](W) writer structure"]
impl crate::Writable for Pfsra9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRA9 to value 0"]
impl crate::Resettable for Pfsra9Spec {}
