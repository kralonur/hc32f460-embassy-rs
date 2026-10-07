#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<Cr3Spec>;
#[doc = "Field `LRCEN` reader - desc LRCEN"]
pub type LrcenR = crate::BitReader;
#[doc = "Field `LRCEN` writer - desc LRCEN"]
pub type LrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCKSEL` reader - desc RCKSEL"]
pub type RckselR = crate::BitReader;
#[doc = "Field `RCKSEL` writer - desc RCKSEL"]
pub type RckselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - desc LRCEN"]
    #[inline(always)]
    pub fn lrcen(&self) -> LrcenR {
        LrcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - desc RCKSEL"]
    #[inline(always)]
    pub fn rcksel(&self) -> RckselR {
        RckselR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - desc LRCEN"]
    #[inline(always)]
    pub fn lrcen(&mut self) -> LrcenW<'_, Cr3Spec> {
        LrcenW::new(self, 4)
    }
    #[doc = "Bit 7 - desc RCKSEL"]
    #[inline(always)]
    pub fn rcksel(&mut self) -> RckselW<'_, Cr3Spec> {
        RckselW::new(self, 7)
    }
}
#[doc = "desc CR3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for Cr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {}
