#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `EOCAF` reader - desc EOCAF"]
pub type EocafR = crate::BitReader;
#[doc = "Field `EOCAF` writer - desc EOCAF"]
pub type EocafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCBF` reader - desc EOCBF"]
pub type EocbfR = crate::BitReader;
#[doc = "Field `EOCBF` writer - desc EOCBF"]
pub type EocbfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc EOCAF"]
    #[inline(always)]
    pub fn eocaf(&self) -> EocafR {
        EocafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc EOCBF"]
    #[inline(always)]
    pub fn eocbf(&self) -> EocbfR {
        EocbfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc EOCAF"]
    #[inline(always)]
    pub fn eocaf(&mut self) -> EocafW<'_, IsrSpec> {
        EocafW::new(self, 0)
    }
    #[doc = "Bit 1 - desc EOCBF"]
    #[inline(always)]
    pub fn eocbf(&mut self) -> EocbfW<'_, IsrSpec> {
        EocbfW::new(self, 1)
    }
}
#[doc = "desc ISR\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
