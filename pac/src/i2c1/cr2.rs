#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `STARTIE` reader - desc STARTIE"]
pub type StartieR = crate::BitReader;
#[doc = "Field `STARTIE` writer - desc STARTIE"]
pub type StartieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR0IE` reader - desc SLADDR0IE"]
pub type Sladdr0ieR = crate::BitReader;
#[doc = "Field `SLADDR0IE` writer - desc SLADDR0IE"]
pub type Sladdr0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR1IE` reader - desc SLADDR1IE"]
pub type Sladdr1ieR = crate::BitReader;
#[doc = "Field `SLADDR1IE` writer - desc SLADDR1IE"]
pub type Sladdr1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENDIE` reader - desc TENDIE"]
pub type TendieR = crate::BitReader;
#[doc = "Field `TENDIE` writer - desc TENDIE"]
pub type TendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPIE` reader - desc STOPIE"]
pub type StopieR = crate::BitReader;
#[doc = "Field `STOPIE` writer - desc STOPIE"]
pub type StopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFULLIE` reader - desc RFULLIE"]
pub type RfullieR = crate::BitReader;
#[doc = "Field `RFULLIE` writer - desc RFULLIE"]
pub type RfullieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPTYIE` reader - desc TEMPTYIE"]
pub type TemptyieR = crate::BitReader;
#[doc = "Field `TEMPTYIE` writer - desc TEMPTYIE"]
pub type TemptyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOIE` reader - desc ARLOIE"]
pub type ArloieR = crate::BitReader;
#[doc = "Field `ARLOIE` writer - desc ARLOIE"]
pub type ArloieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKIE` reader - desc NACKIE"]
pub type NackieR = crate::BitReader;
#[doc = "Field `NACKIE` writer - desc NACKIE"]
pub type NackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUTIE` reader - desc TMOUTIE"]
pub type TmoutieR = crate::BitReader;
#[doc = "Field `TMOUTIE` writer - desc TMOUTIE"]
pub type TmoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALLIE` reader - desc GENCALLIE"]
pub type GencallieR = crate::BitReader;
#[doc = "Field `GENCALLIE` writer - desc GENCALLIE"]
pub type GencallieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEFAULTIE` reader - desc SMBDEFAULTIE"]
pub type SmbdefaultieR = crate::BitReader;
#[doc = "Field `SMBDEFAULTIE` writer - desc SMBDEFAULTIE"]
pub type SmbdefaultieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHOSTIE` reader - desc SMBHOSTIE"]
pub type SmbhostieR = crate::BitReader;
#[doc = "Field `SMBHOSTIE` writer - desc SMBHOSTIE"]
pub type SmbhostieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALRTIE` reader - desc SMBALRTIE"]
pub type SmbalrtieR = crate::BitReader;
#[doc = "Field `SMBALRTIE` writer - desc SMBALRTIE"]
pub type SmbalrtieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc STARTIE"]
    #[inline(always)]
    pub fn startie(&self) -> StartieR {
        StartieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SLADDR0IE"]
    #[inline(always)]
    pub fn sladdr0ie(&self) -> Sladdr0ieR {
        Sladdr0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SLADDR1IE"]
    #[inline(always)]
    pub fn sladdr1ie(&self) -> Sladdr1ieR {
        Sladdr1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TENDIE"]
    #[inline(always)]
    pub fn tendie(&self) -> TendieR {
        TendieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STOPIE"]
    #[inline(always)]
    pub fn stopie(&self) -> StopieR {
        StopieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RFULLIE"]
    #[inline(always)]
    pub fn rfullie(&self) -> RfullieR {
        RfullieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TEMPTYIE"]
    #[inline(always)]
    pub fn temptyie(&self) -> TemptyieR {
        TemptyieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ARLOIE"]
    #[inline(always)]
    pub fn arloie(&self) -> ArloieR {
        ArloieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - desc NACKIE"]
    #[inline(always)]
    pub fn nackie(&self) -> NackieR {
        NackieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TMOUTIE"]
    #[inline(always)]
    pub fn tmoutie(&self) -> TmoutieR {
        TmoutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 20 - desc GENCALLIE"]
    #[inline(always)]
    pub fn gencallie(&self) -> GencallieR {
        GencallieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTIE"]
    #[inline(always)]
    pub fn smbdefaultie(&self) -> SmbdefaultieR {
        SmbdefaultieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc SMBHOSTIE"]
    #[inline(always)]
    pub fn smbhostie(&self) -> SmbhostieR {
        SmbhostieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc SMBALRTIE"]
    #[inline(always)]
    pub fn smbalrtie(&self) -> SmbalrtieR {
        SmbalrtieR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc STARTIE"]
    #[inline(always)]
    pub fn startie(&mut self) -> StartieW<'_, Cr2Spec> {
        StartieW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SLADDR0IE"]
    #[inline(always)]
    pub fn sladdr0ie(&mut self) -> Sladdr0ieW<'_, Cr2Spec> {
        Sladdr0ieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SLADDR1IE"]
    #[inline(always)]
    pub fn sladdr1ie(&mut self) -> Sladdr1ieW<'_, Cr2Spec> {
        Sladdr1ieW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TENDIE"]
    #[inline(always)]
    pub fn tendie(&mut self) -> TendieW<'_, Cr2Spec> {
        TendieW::new(self, 3)
    }
    #[doc = "Bit 4 - desc STOPIE"]
    #[inline(always)]
    pub fn stopie(&mut self) -> StopieW<'_, Cr2Spec> {
        StopieW::new(self, 4)
    }
    #[doc = "Bit 6 - desc RFULLIE"]
    #[inline(always)]
    pub fn rfullie(&mut self) -> RfullieW<'_, Cr2Spec> {
        RfullieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TEMPTYIE"]
    #[inline(always)]
    pub fn temptyie(&mut self) -> TemptyieW<'_, Cr2Spec> {
        TemptyieW::new(self, 7)
    }
    #[doc = "Bit 9 - desc ARLOIE"]
    #[inline(always)]
    pub fn arloie(&mut self) -> ArloieW<'_, Cr2Spec> {
        ArloieW::new(self, 9)
    }
    #[doc = "Bit 12 - desc NACKIE"]
    #[inline(always)]
    pub fn nackie(&mut self) -> NackieW<'_, Cr2Spec> {
        NackieW::new(self, 12)
    }
    #[doc = "Bit 14 - desc TMOUTIE"]
    #[inline(always)]
    pub fn tmoutie(&mut self) -> TmoutieW<'_, Cr2Spec> {
        TmoutieW::new(self, 14)
    }
    #[doc = "Bit 20 - desc GENCALLIE"]
    #[inline(always)]
    pub fn gencallie(&mut self) -> GencallieW<'_, Cr2Spec> {
        GencallieW::new(self, 20)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTIE"]
    #[inline(always)]
    pub fn smbdefaultie(&mut self) -> SmbdefaultieW<'_, Cr2Spec> {
        SmbdefaultieW::new(self, 21)
    }
    #[doc = "Bit 22 - desc SMBHOSTIE"]
    #[inline(always)]
    pub fn smbhostie(&mut self) -> SmbhostieW<'_, Cr2Spec> {
        SmbhostieW::new(self, 22)
    }
    #[doc = "Bit 23 - desc SMBALRTIE"]
    #[inline(always)]
    pub fn smbalrtie(&mut self) -> SmbalrtieW<'_, Cr2Spec> {
        SmbalrtieW::new(self, 23)
    }
}
#[doc = "desc CR2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
