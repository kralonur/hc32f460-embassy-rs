#[doc = "Register `STATCLR` writer"]
pub type W = crate::W<StatclrSpec>;
#[doc = "Field `PORTINFCLR` writer - desc PORTINFCLR"]
pub type PortinfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSFCLR` writer - desc PWMSFCLR"]
pub type PwmsfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPFCLR` writer - desc CMPFCLR"]
pub type CmpfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSFCLR` writer - desc OSFCLR"]
pub type OsfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc PORTINFCLR"]
    #[inline(always)]
    pub fn portinfclr(&mut self) -> PortinfclrW<'_, StatclrSpec> {
        PortinfclrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc PWMSFCLR"]
    #[inline(always)]
    pub fn pwmsfclr(&mut self) -> PwmsfclrW<'_, StatclrSpec> {
        PwmsfclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPFCLR"]
    #[inline(always)]
    pub fn cmpfclr(&mut self) -> CmpfclrW<'_, StatclrSpec> {
        CmpfclrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc OSFCLR"]
    #[inline(always)]
    pub fn osfclr(&mut self) -> OsfclrW<'_, StatclrSpec> {
        OsfclrW::new(self, 3)
    }
}
#[doc = "desc STATCLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatclrSpec;
impl crate::RegisterSpec for StatclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`statclr::W`](W) writer structure"]
impl crate::Writable for StatclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATCLR to value 0"]
impl crate::Resettable for StatclrSpec {}
