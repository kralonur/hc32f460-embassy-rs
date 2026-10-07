#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `STARTF` reader - desc STARTF"]
pub type StartfR = crate::BitReader;
#[doc = "Field `STARTF` writer - desc STARTF"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR0F` reader - desc SLADDR0F"]
pub type Sladdr0fR = crate::BitReader;
#[doc = "Field `SLADDR0F` writer - desc SLADDR0F"]
pub type Sladdr0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLADDR1F` reader - desc SLADDR1F"]
pub type Sladdr1fR = crate::BitReader;
#[doc = "Field `SLADDR1F` writer - desc SLADDR1F"]
pub type Sladdr1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TENDF` reader - desc TENDF"]
pub type TendfR = crate::BitReader;
#[doc = "Field `TENDF` writer - desc TENDF"]
pub type TendfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - desc STOPF"]
pub type StopfR = crate::BitReader;
#[doc = "Field `STOPF` writer - desc STOPF"]
pub type StopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFULLF` reader - desc RFULLF"]
pub type RfullfR = crate::BitReader;
#[doc = "Field `RFULLF` writer - desc RFULLF"]
pub type RfullfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPTYF` reader - desc TEMPTYF"]
pub type TemptyfR = crate::BitReader;
#[doc = "Field `TEMPTYF` writer - desc TEMPTYF"]
pub type TemptyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARLOF` reader - desc ARLOF"]
pub type ArlofR = crate::BitReader;
#[doc = "Field `ARLOF` writer - desc ARLOF"]
pub type ArlofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKRF` reader - desc ACKRF"]
pub type AckrfR = crate::BitReader;
#[doc = "Field `ACKRF` writer - desc ACKRF"]
pub type AckrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKF` reader - desc NACKF"]
pub type NackfR = crate::BitReader;
#[doc = "Field `NACKF` writer - desc NACKF"]
pub type NackfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMOUTF` reader - desc TMOUTF"]
pub type TmoutfR = crate::BitReader;
#[doc = "Field `TMOUTF` writer - desc TMOUTF"]
pub type TmoutfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSL` reader - desc MSL"]
pub type MslR = crate::BitReader;
#[doc = "Field `MSL` writer - desc MSL"]
pub type MslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSY` reader - desc BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - desc BUSY"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRA` reader - desc TRA"]
pub type TraR = crate::BitReader;
#[doc = "Field `TRA` writer - desc TRA"]
pub type TraW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENCALLF` reader - desc GENCALLF"]
pub type GencallfR = crate::BitReader;
#[doc = "Field `GENCALLF` writer - desc GENCALLF"]
pub type GencallfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBDEFAULTF` reader - desc SMBDEFAULTF"]
pub type SmbdefaultfR = crate::BitReader;
#[doc = "Field `SMBDEFAULTF` writer - desc SMBDEFAULTF"]
pub type SmbdefaultfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBHOSTF` reader - desc SMBHOSTF"]
pub type SmbhostfR = crate::BitReader;
#[doc = "Field `SMBHOSTF` writer - desc SMBHOSTF"]
pub type SmbhostfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMBALRTF` reader - desc SMBALRTF"]
pub type SmbalrtfR = crate::BitReader;
#[doc = "Field `SMBALRTF` writer - desc SMBALRTF"]
pub type SmbalrtfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc STARTF"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc SLADDR0F"]
    #[inline(always)]
    pub fn sladdr0f(&self) -> Sladdr0fR {
        Sladdr0fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc SLADDR1F"]
    #[inline(always)]
    pub fn sladdr1f(&self) -> Sladdr1fR {
        Sladdr1fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TENDF"]
    #[inline(always)]
    pub fn tendf(&self) -> TendfR {
        TendfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc STOPF"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - desc RFULLF"]
    #[inline(always)]
    pub fn rfullf(&self) -> RfullfR {
        RfullfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TEMPTYF"]
    #[inline(always)]
    pub fn temptyf(&self) -> TemptyfR {
        TemptyfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc ARLOF"]
    #[inline(always)]
    pub fn arlof(&self) -> ArlofR {
        ArlofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc ACKRF"]
    #[inline(always)]
    pub fn ackrf(&self) -> AckrfR {
        AckrfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc NACKF"]
    #[inline(always)]
    pub fn nackf(&self) -> NackfR {
        NackfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - desc TMOUTF"]
    #[inline(always)]
    pub fn tmoutf(&self) -> TmoutfR {
        TmoutfR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - desc MSL"]
    #[inline(always)]
    pub fn msl(&self) -> MslR {
        MslR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - desc TRA"]
    #[inline(always)]
    pub fn tra(&self) -> TraR {
        TraR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - desc GENCALLF"]
    #[inline(always)]
    pub fn gencallf(&self) -> GencallfR {
        GencallfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTF"]
    #[inline(always)]
    pub fn smbdefaultf(&self) -> SmbdefaultfR {
        SmbdefaultfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - desc SMBHOSTF"]
    #[inline(always)]
    pub fn smbhostf(&self) -> SmbhostfR {
        SmbhostfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - desc SMBALRTF"]
    #[inline(always)]
    pub fn smbalrtf(&self) -> SmbalrtfR {
        SmbalrtfR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc STARTF"]
    #[inline(always)]
    pub fn startf(&mut self) -> StartfW<'_, SrSpec> {
        StartfW::new(self, 0)
    }
    #[doc = "Bit 1 - desc SLADDR0F"]
    #[inline(always)]
    pub fn sladdr0f(&mut self) -> Sladdr0fW<'_, SrSpec> {
        Sladdr0fW::new(self, 1)
    }
    #[doc = "Bit 2 - desc SLADDR1F"]
    #[inline(always)]
    pub fn sladdr1f(&mut self) -> Sladdr1fW<'_, SrSpec> {
        Sladdr1fW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TENDF"]
    #[inline(always)]
    pub fn tendf(&mut self) -> TendfW<'_, SrSpec> {
        TendfW::new(self, 3)
    }
    #[doc = "Bit 4 - desc STOPF"]
    #[inline(always)]
    pub fn stopf(&mut self) -> StopfW<'_, SrSpec> {
        StopfW::new(self, 4)
    }
    #[doc = "Bit 6 - desc RFULLF"]
    #[inline(always)]
    pub fn rfullf(&mut self) -> RfullfW<'_, SrSpec> {
        RfullfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TEMPTYF"]
    #[inline(always)]
    pub fn temptyf(&mut self) -> TemptyfW<'_, SrSpec> {
        TemptyfW::new(self, 7)
    }
    #[doc = "Bit 9 - desc ARLOF"]
    #[inline(always)]
    pub fn arlof(&mut self) -> ArlofW<'_, SrSpec> {
        ArlofW::new(self, 9)
    }
    #[doc = "Bit 10 - desc ACKRF"]
    #[inline(always)]
    pub fn ackrf(&mut self) -> AckrfW<'_, SrSpec> {
        AckrfW::new(self, 10)
    }
    #[doc = "Bit 12 - desc NACKF"]
    #[inline(always)]
    pub fn nackf(&mut self) -> NackfW<'_, SrSpec> {
        NackfW::new(self, 12)
    }
    #[doc = "Bit 14 - desc TMOUTF"]
    #[inline(always)]
    pub fn tmoutf(&mut self) -> TmoutfW<'_, SrSpec> {
        TmoutfW::new(self, 14)
    }
    #[doc = "Bit 16 - desc MSL"]
    #[inline(always)]
    pub fn msl(&mut self) -> MslW<'_, SrSpec> {
        MslW::new(self, 16)
    }
    #[doc = "Bit 17 - desc BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, SrSpec> {
        BusyW::new(self, 17)
    }
    #[doc = "Bit 18 - desc TRA"]
    #[inline(always)]
    pub fn tra(&mut self) -> TraW<'_, SrSpec> {
        TraW::new(self, 18)
    }
    #[doc = "Bit 20 - desc GENCALLF"]
    #[inline(always)]
    pub fn gencallf(&mut self) -> GencallfW<'_, SrSpec> {
        GencallfW::new(self, 20)
    }
    #[doc = "Bit 21 - desc SMBDEFAULTF"]
    #[inline(always)]
    pub fn smbdefaultf(&mut self) -> SmbdefaultfW<'_, SrSpec> {
        SmbdefaultfW::new(self, 21)
    }
    #[doc = "Bit 22 - desc SMBHOSTF"]
    #[inline(always)]
    pub fn smbhostf(&mut self) -> SmbhostfW<'_, SrSpec> {
        SmbhostfW::new(self, 22)
    }
    #[doc = "Bit 23 - desc SMBALRTF"]
    #[inline(always)]
    pub fn smbalrtf(&mut self) -> SmbalrtfW<'_, SrSpec> {
        SmbalrtfW::new(self, 23)
    }
}
#[doc = "desc SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
