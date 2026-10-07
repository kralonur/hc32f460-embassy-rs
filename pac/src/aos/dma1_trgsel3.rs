#[doc = "Register `DMA1_TRGSEL3` reader"]
pub type R = crate::R<Dma1Trgsel3Spec>;
#[doc = "Register `DMA1_TRGSEL3` writer"]
pub type W = crate::W<Dma1Trgsel3Spec>;
#[doc = "Field `TRGSEL` reader - desc TRGSEL"]
pub type TrgselR = crate::FieldReader<u16>;
#[doc = "Field `TRGSEL` writer - desc TRGSEL"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `COMEN` reader - desc COMEN"]
pub type ComenR = crate::FieldReader;
#[doc = "Field `COMEN` writer - desc COMEN"]
pub type ComenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&self) -> ComenR {
        ComenR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TRGSEL"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TrgselW<'_, Dma1Trgsel3Spec> {
        TrgselW::new(self, 0)
    }
    #[doc = "Bits 30:31 - desc COMEN"]
    #[inline(always)]
    pub fn comen(&mut self) -> ComenW<'_, Dma1Trgsel3Spec> {
        ComenW::new(self, 30)
    }
}
#[doc = "desc DMA1_TRGSEL3\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1_trgsel3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1_trgsel3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1Trgsel3Spec;
impl crate::RegisterSpec for Dma1Trgsel3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1_trgsel3::R`](R) reader structure"]
impl crate::Readable for Dma1Trgsel3Spec {}
#[doc = "`write(|w| ..)` method takes [`dma1_trgsel3::W`](W) writer structure"]
impl crate::Writable for Dma1Trgsel3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1_TRGSEL3 to value 0x01ff"]
impl crate::Resettable for Dma1Trgsel3Spec {
    const RESET_VALUE: u32 = 0x01ff;
}
