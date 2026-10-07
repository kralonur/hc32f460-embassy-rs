#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `STARTFCLR` writer - desc STARTFCLR"]
pub type StartfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR0FCLR` writer - desc SLADDR0FCLR"]
pub type Sladdr0fclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR1FCLR` writer - desc SLADDR1FCLR"]
pub type Sladdr1fclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENDFCLR` writer - desc TENDFCLR"]
pub type TendfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPFCLR` writer - desc STOPFCLR"]
pub type StopfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFULLFCLR` writer - desc RFULLFCLR"]
pub type RfullfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPTYFCLR` writer - desc TEMPTYFCLR"]
pub type TemptyfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOFCLR` writer - desc ARLOFCLR"]
pub type ArlofclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKFCLR` writer - desc NACKFCLR"]
pub type NackfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUTFCLR` writer - desc TMOUTFCLR"]
pub type TmoutfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALLFCLR` writer - desc GENCALLFCLR"]
pub type GencallfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEFAULTFCLR` writer - desc SMBDEFAULTFCLR"]
pub type SmbdefaultfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHOSTFCLR` writer - desc SMBHOSTFCLR"]
pub type SmbhostfclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALRTFCLR` writer - desc SMBALRTFCLR"]
pub type SmbalrtfclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - desc STARTFCLR"]
    #[inline(always)]
    pub fn startfclr(&mut self) -> StartfclrW<'_, ClrSpec> {
        StartfclrW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SLADDR0FCLR"]
    #[inline(always)]
    pub fn sladdr0fclr(&mut self) -> Sladdr0fclrW<'_, ClrSpec> {
        Sladdr0fclrW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SLADDR1FCLR"]
    #[inline(always)]
    pub fn sladdr1fclr(&mut self) -> Sladdr1fclrW<'_, ClrSpec> {
        Sladdr1fclrW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TENDFCLR"]
    #[inline(always)]
    pub fn tendfclr(&mut self) -> TendfclrW<'_, ClrSpec> {
        TendfclrW::new(self, 3)
    }
    #[doc = "Bit 4 - desc STOPFCLR"]
    #[inline(always)]
    pub fn stopfclr(&mut self) -> StopfclrW<'_, ClrSpec> {
        StopfclrW::new(self, 4)
    }
    #[doc = "Bit 6 - desc RFULLFCLR"]
    #[inline(always)]
    pub fn rfullfclr(&mut self) -> RfullfclrW<'_, ClrSpec> {
        RfullfclrW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TEMPTYFCLR"]
    #[inline(always)]
    pub fn temptyfclr(&mut self) -> TemptyfclrW<'_, ClrSpec> {
        TemptyfclrW::new(self, 7)
    }
    #[doc = "Bit 9 - desc ARLOFCLR"]
    #[inline(always)]
    pub fn arlofclr(&mut self) -> ArlofclrW<'_, ClrSpec> {
        ArlofclrW::new(self, 9)
    }
    #[doc = "Bit 12 - desc NACKFCLR"]
    #[inline(always)]
    pub fn nackfclr(&mut self) -> NackfclrW<'_, ClrSpec> {
        NackfclrW::new(self, 12)
    }
    #[doc = "Bit 14 - desc TMOUTFCLR"]
    #[inline(always)]
    pub fn tmoutfclr(&mut self) -> TmoutfclrW<'_, ClrSpec> {
        TmoutfclrW::new(self, 14)
    }
    #[doc = "Bit 20 - desc GENCALLFCLR"]
    #[inline(always)]
    pub fn gencallfclr(&mut self) -> GencallfclrW<'_, ClrSpec> {
        GencallfclrW::new(self, 20)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTFCLR"]
    #[inline(always)]
    pub fn smbdefaultfclr(&mut self) -> SmbdefaultfclrW<'_, ClrSpec> {
        SmbdefaultfclrW::new(self, 21)
    }
    #[doc = "Bit 22 - desc SMBHOSTFCLR"]
    #[inline(always)]
    pub fn smbhostfclr(&mut self) -> SmbhostfclrW<'_, ClrSpec> {
        SmbhostfclrW::new(self, 22)
    }
    #[doc = "Bit 23 - desc SMBALRTFCLR"]
    #[inline(always)]
    pub fn smbalrtfclr(&mut self) -> SmbalrtfclrW<'_, ClrSpec> {
        SmbalrtfclrW::new(self, 23)
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
