#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `RTOE` reader - desc RTOE"]
pub type RtoeR = crate::BitReader;
#[doc = "Field `RTOE` writer - desc RTOE"]
pub type RtoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTOIE` reader - desc RTOIE"]
pub type RtoieR = crate::BitReader;
#[doc = "Field `RTOIE` writer - desc RTOIE"]
pub type RtoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - desc RE"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - desc RE"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - desc TE"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - desc TE"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLME` reader - desc SLME"]
pub type SlmeR = crate::BitReader;
#[doc = "Field `SLME` writer - desc SLME"]
pub type SlmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - desc RIE"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - desc RIE"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - desc TCIE"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - desc TCIE"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - desc TXEIE"]
pub type TxeieR = crate::BitReader;
#[doc = "Field `TXEIE` writer - desc TXEIE"]
pub type TxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - desc PS"]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - desc PS"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCE` reader - desc PCE"]
pub type PceR = crate::BitReader;
#[doc = "Field `PCE` writer - desc PCE"]
pub type PceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - desc M"]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - desc M"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVER8` reader - desc OVER8"]
pub type Over8R = crate::BitReader;
#[doc = "Field `OVER8` writer - desc OVER8"]
pub type Over8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPE` writer - desc CPE"]
pub type CpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFE` writer - desc CFE"]
pub type CfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORE` writer - desc CORE"]
pub type CoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRTOF` writer - desc CRTOF"]
pub type CrtofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MS` reader - desc MS"]
pub type MsR = crate::BitReader;
#[doc = "Field `MS` writer - desc MS"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ML` reader - desc ML"]
pub type MlR = crate::BitReader;
#[doc = "Field `ML` writer - desc ML"]
pub type MlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBME` reader - desc FBME"]
pub type FbmeR = crate::BitReader;
#[doc = "Field `FBME` writer - desc FBME"]
pub type FbmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFE` reader - desc NFE"]
pub type NfeR = crate::BitReader;
#[doc = "Field `NFE` writer - desc NFE"]
pub type NfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBS` reader - desc SBS"]
pub type SbsR = crate::BitReader;
#[doc = "Field `SBS` writer - desc SBS"]
pub type SbsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc RTOE"]
    #[inline(always)]
    pub fn rtoe(&self) -> RtoeR {
        RtoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&self) -> RtoieR {
        RtoieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc RE"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc TE"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc SLME"]
    #[inline(always)]
    pub fn slme(&self) -> SlmeR {
        SlmeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc RIE"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&self) -> TxeieR {
        TxeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - desc PS"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc PCE"]
    #[inline(always)]
    pub fn pce(&self) -> PceR {
        PceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - desc M"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - desc OVER8"]
    #[inline(always)]
    pub fn over8(&self) -> Over8R {
        Over8R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - desc MS"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - desc ML"]
    #[inline(always)]
    pub fn ml(&self) -> MlR {
        MlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - desc FBME"]
    #[inline(always)]
    pub fn fbme(&self) -> FbmeR {
        FbmeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - desc NFE"]
    #[inline(always)]
    pub fn nfe(&self) -> NfeR {
        NfeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc SBS"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc RTOE"]
    #[inline(always)]
    pub fn rtoe(&mut self) -> RtoeW<'_, Cr1Spec> {
        RtoeW::new(self, 0)
    }
    #[doc = "Bit 1 - desc RTOIE"]
    #[inline(always)]
    pub fn rtoie(&mut self) -> RtoieW<'_, Cr1Spec> {
        RtoieW::new(self, 1)
    }
    #[doc = "Bit 2 - desc RE"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<'_, Cr1Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - desc TE"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<'_, Cr1Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - desc SLME"]
    #[inline(always)]
    pub fn slme(&mut self) -> SlmeW<'_, Cr1Spec> {
        SlmeW::new(self, 4)
    }
    #[doc = "Bit 5 - desc RIE"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, Cr1Spec> {
        RieW::new(self, 5)
    }
    #[doc = "Bit 6 - desc TCIE"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, Cr1Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TXEIE"]
    #[inline(always)]
    pub fn txeie(&mut self) -> TxeieW<'_, Cr1Spec> {
        TxeieW::new(self, 7)
    }
    #[doc = "Bit 9 - desc PS"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, Cr1Spec> {
        PsW::new(self, 9)
    }
    #[doc = "Bit 10 - desc PCE"]
    #[inline(always)]
    pub fn pce(&mut self) -> PceW<'_, Cr1Spec> {
        PceW::new(self, 10)
    }
    #[doc = "Bit 12 - desc M"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, Cr1Spec> {
        MW::new(self, 12)
    }
    #[doc = "Bit 15 - desc OVER8"]
    #[inline(always)]
    pub fn over8(&mut self) -> Over8W<'_, Cr1Spec> {
        Over8W::new(self, 15)
    }
    #[doc = "Bit 16 - desc CPE"]
    #[inline(always)]
    pub fn cpe(&mut self) -> CpeW<'_, Cr1Spec> {
        CpeW::new(self, 16)
    }
    #[doc = "Bit 17 - desc CFE"]
    #[inline(always)]
    pub fn cfe(&mut self) -> CfeW<'_, Cr1Spec> {
        CfeW::new(self, 17)
    }
    #[doc = "Bit 19 - desc CORE"]
    #[inline(always)]
    pub fn core(&mut self) -> CoreW<'_, Cr1Spec> {
        CoreW::new(self, 19)
    }
    #[doc = "Bit 20 - desc CRTOF"]
    #[inline(always)]
    pub fn crtof(&mut self) -> CrtofW<'_, Cr1Spec> {
        CrtofW::new(self, 20)
    }
    #[doc = "Bit 24 - desc MS"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Cr1Spec> {
        MsW::new(self, 24)
    }
    #[doc = "Bit 28 - desc ML"]
    #[inline(always)]
    pub fn ml(&mut self) -> MlW<'_, Cr1Spec> {
        MlW::new(self, 28)
    }
    #[doc = "Bit 29 - desc FBME"]
    #[inline(always)]
    pub fn fbme(&mut self) -> FbmeW<'_, Cr1Spec> {
        FbmeW::new(self, 29)
    }
    #[doc = "Bit 30 - desc NFE"]
    #[inline(always)]
    pub fn nfe(&mut self) -> NfeW<'_, Cr1Spec> {
        NfeW::new(self, 30)
    }
    #[doc = "Bit 31 - desc SBS"]
    #[inline(always)]
    pub fn sbs(&mut self) -> SbsW<'_, Cr1Spec> {
        SbsW::new(self, 31)
    }
}
#[doc = "desc CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0x8000_0000"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
