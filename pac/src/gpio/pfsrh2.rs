#[doc = "Register `PFSRH2` reader"]
pub type R = crate::R<Pfsrh2Spec>;
#[doc = "Register `PFSRH2` writer"]
pub type W = crate::W<Pfsrh2Spec>;
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
    pub fn fsel(&mut self) -> FselW<'_, Pfsrh2Spec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 8 - desc BFE"]
    #[inline(always)]
    pub fn bfe(&mut self) -> BfeW<'_, Pfsrh2Spec> {
        BfeW::new(self, 8)
    }
}
#[doc = "desc PFSRH2\n\nYou can [`read`](crate::Reg::read) this register and get [`pfsrh2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfsrh2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pfsrh2Spec;
impl crate::RegisterSpec for Pfsrh2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pfsrh2::R`](R) reader structure"]
impl crate::Readable for Pfsrh2Spec {}
#[doc = "`write(|w| ..)` method takes [`pfsrh2::W`](W) writer structure"]
impl crate::Writable for Pfsrh2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PFSRH2 to value 0x0d"]
impl crate::Resettable for Pfsrh2Spec {
    const RESET_VALUE: u16 = 0x0d;
}
