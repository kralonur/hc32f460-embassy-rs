#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `ERRFCLR` writer - desc ERRFCLR"]
pub type ErrfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MENDFCLR` writer - desc MENDFCLR"]
pub type MendfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFCLR` writer - desc OVFCLR"]
pub type OvfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc ERRFCLR"]
    #[inline(always)]
    pub fn errfclr(&mut self) -> ErrfclrW<'_, ClrSpec> {
        ErrfclrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc MENDFCLR"]
    #[inline(always)]
    pub fn mendfclr(&mut self) -> MendfclrW<'_, ClrSpec> {
        MendfclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc OVFCLR"]
    #[inline(always)]
    pub fn ovfclr(&mut self) -> OvfclrW<'_, ClrSpec> {
        OvfclrW::new(self, 2)
    }
}
#[doc = "desc CLR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {}
