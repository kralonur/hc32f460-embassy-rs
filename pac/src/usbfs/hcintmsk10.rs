#[doc = "Register `HCINTMSK10` reader"]
pub type R = crate::R<Hcintmsk10Spec>;
#[doc = "Register `HCINTMSK10` writer"]
pub type W = crate::W<Hcintmsk10Spec>;
#[doc = "Field `XFRCM` reader - desc XFRCM"]
pub type XfrcmR = crate::BitReader;
#[doc = "Field `XFRCM` writer - desc XFRCM"]
pub type XfrcmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHM` reader - desc CHHM"]
pub type ChhmR = crate::BitReader;
#[doc = "Field `CHHM` writer - desc CHHM"]
pub type ChhmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLM` reader - desc STALLM"]
pub type StallmR = crate::BitReader;
#[doc = "Field `STALLM` writer - desc STALLM"]
pub type StallmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKM` reader - desc NAKM"]
pub type NakmR = crate::BitReader;
#[doc = "Field `NAKM` writer - desc NAKM"]
pub type NakmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKM` reader - desc ACKM"]
pub type AckmR = crate::BitReader;
#[doc = "Field `ACKM` writer - desc ACKM"]
pub type AckmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRM` reader - desc TXERRM"]
pub type TxerrmR = crate::BitReader;
#[doc = "Field `TXERRM` writer - desc TXERRM"]
pub type TxerrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERRM` reader - desc BBERRM"]
pub type BberrmR = crate::BitReader;
#[doc = "Field `BBERRM` writer - desc BBERRM"]
pub type BberrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMORM` reader - desc FRMORM"]
pub type FrmormR = crate::BitReader;
#[doc = "Field `FRMORM` writer - desc FRMORM"]
pub type FrmormW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRM` reader - desc DTERRM"]
pub type DterrmR = crate::BitReader;
#[doc = "Field `DTERRM` writer - desc DTERRM"]
pub type DterrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XfrcmR {
        XfrcmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CHHM"]
    #[inline(always)]
    pub fn chhm(&self) -> ChhmR {
        ChhmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STALLM"]
    #[inline(always)]
    pub fn stallm(&self) -> StallmR {
        StallmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc NAKM"]
    #[inline(always)]
    pub fn nakm(&self) -> NakmR {
        NakmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ACKM"]
    #[inline(always)]
    pub fn ackm(&self) -> AckmR {
        AckmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXERRM"]
    #[inline(always)]
    pub fn txerrm(&self) -> TxerrmR {
        TxerrmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BBERRM"]
    #[inline(always)]
    pub fn bberrm(&self) -> BberrmR {
        BberrmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc FRMORM"]
    #[inline(always)]
    pub fn frmorm(&self) -> FrmormR {
        FrmormR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DTERRM"]
    #[inline(always)]
    pub fn dterrm(&self) -> DterrmR {
        DterrmR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&mut self) -> XfrcmW<'_, Hcintmsk10Spec> {
        XfrcmW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CHHM"]
    #[inline(always)]
    pub fn chhm(&mut self) -> ChhmW<'_, Hcintmsk10Spec> {
        ChhmW::new(self, 1)
    }
    #[doc = "Bit 3 - desc STALLM"]
    #[inline(always)]
    pub fn stallm(&mut self) -> StallmW<'_, Hcintmsk10Spec> {
        StallmW::new(self, 3)
    }
    #[doc = "Bit 4 - desc NAKM"]
    #[inline(always)]
    pub fn nakm(&mut self) -> NakmW<'_, Hcintmsk10Spec> {
        NakmW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ACKM"]
    #[inline(always)]
    pub fn ackm(&mut self) -> AckmW<'_, Hcintmsk10Spec> {
        AckmW::new(self, 5)
    }
    #[doc = "Bit 7 - desc TXERRM"]
    #[inline(always)]
    pub fn txerrm(&mut self) -> TxerrmW<'_, Hcintmsk10Spec> {
        TxerrmW::new(self, 7)
    }
    #[doc = "Bit 8 - desc BBERRM"]
    #[inline(always)]
    pub fn bberrm(&mut self) -> BberrmW<'_, Hcintmsk10Spec> {
        BberrmW::new(self, 8)
    }
    #[doc = "Bit 9 - desc FRMORM"]
    #[inline(always)]
    pub fn frmorm(&mut self) -> FrmormW<'_, Hcintmsk10Spec> {
        FrmormW::new(self, 9)
    }
    #[doc = "Bit 10 - desc DTERRM"]
    #[inline(always)]
    pub fn dterrm(&mut self) -> DterrmW<'_, Hcintmsk10Spec> {
        DterrmW::new(self, 10)
    }
}
#[doc = "desc HCINTMSK10\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcintmsk10Spec;
impl crate::RegisterSpec for Hcintmsk10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk10::R`](R) reader structure"]
impl crate::Readable for Hcintmsk10Spec {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk10::W`](W) writer structure"]
impl crate::Writable for Hcintmsk10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCINTMSK10 to value 0"]
impl crate::Resettable for Hcintmsk10Spec {}
