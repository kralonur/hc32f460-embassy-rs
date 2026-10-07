#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `PORTININTEN` reader - desc PORTININTEN"]
pub type PortinintenR = crate::BitReader;
#[doc = "Field `PORTININTEN` writer - desc PORTININTEN"]
pub type PortinintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSINTEN` reader - desc PWMSINTEN"]
pub type PwmsintenR = crate::BitReader;
#[doc = "Field `PWMSINTEN` writer - desc PWMSINTEN"]
pub type PwmsintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPINTEN` reader - desc CMPINTEN"]
pub type CmpintenR = crate::BitReader;
#[doc = "Field `CMPINTEN` writer - desc CMPINTEN"]
pub type CmpintenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSINTEN` reader - desc OSINTEN"]
pub type OsintenR = crate::BitReader;
#[doc = "Field `OSINTEN` writer - desc OSINTEN"]
pub type OsintenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PORTININTEN"]
    #[inline(always)]
    pub fn portininten(&self) -> PortinintenR {
        PortinintenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc PWMSINTEN"]
    #[inline(always)]
    pub fn pwmsinten(&self) -> PwmsintenR {
        PwmsintenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPINTEN"]
    #[inline(always)]
    pub fn cmpinten(&self) -> CmpintenR {
        CmpintenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc OSINTEN"]
    #[inline(always)]
    pub fn osinten(&self) -> OsintenR {
        OsintenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PORTININTEN"]
    #[inline(always)]
    pub fn portininten(&mut self) -> PortinintenW<'_, IntenSpec> {
        PortinintenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PWMSINTEN"]
    #[inline(always)]
    pub fn pwmsinten(&mut self) -> PwmsintenW<'_, IntenSpec> {
        PwmsintenW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPINTEN"]
    #[inline(always)]
    pub fn cmpinten(&mut self) -> CmpintenW<'_, IntenSpec> {
        CmpintenW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OSINTEN"]
    #[inline(always)]
    pub fn osinten(&mut self) -> OsintenW<'_, IntenSpec> {
        OsintenW::new(self, 3)
    }
}
#[doc = "desc INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
