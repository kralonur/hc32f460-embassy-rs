#[doc = "Register `HCINT9` reader"]
pub type R = crate::R<Hcint9Spec>;
#[doc = "Register `HCINT9` writer"]
pub type W = crate::W<Hcint9Spec>;
#[doc = "Field `XFRC` reader - desc XFRC"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - desc XFRC"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHH` reader - desc CHH"]
pub type ChhR = crate::BitReader;
#[doc = "Field `CHH` writer - desc CHH"]
pub type ChhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - desc STALL"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - desc STALL"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - desc NAK"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - desc NAK"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - desc ACK"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - desc ACK"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - desc TXERR"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - desc TXERR"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBERR` reader - desc BBERR"]
pub type BberrR = crate::BitReader;
#[doc = "Field `BBERR` writer - desc BBERR"]
pub type BberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOR` reader - desc FRMOR"]
pub type FrmorR = crate::BitReader;
#[doc = "Field `FRMOR` writer - desc FRMOR"]
pub type FrmorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERR` reader - desc DTERR"]
pub type DterrR = crate::BitReader;
#[doc = "Field `DTERR` writer - desc DTERR"]
pub type DterrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CHH"]
    #[inline(always)]
    pub fn chh(&self) -> ChhR {
        ChhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - desc STALL"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - desc NAK"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - desc ACK"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TXERR"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc BBERR"]
    #[inline(always)]
    pub fn bberr(&self) -> BberrR {
        BberrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc FRMOR"]
    #[inline(always)]
    pub fn frmor(&self) -> FrmorR {
        FrmorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc DTERR"]
    #[inline(always)]
    pub fn dterr(&self) -> DterrR {
        DterrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc XFRC"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<'_, Hcint9Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CHH"]
    #[inline(always)]
    pub fn chh(&mut self) -> ChhW<'_, Hcint9Spec> {
        ChhW::new(self, 1)
    }
    #[doc = "Bit 3 - desc STALL"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Hcint9Spec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - desc NAK"]
    #[inline(always)]
    pub fn nak(&mut self) -> NakW<'_, Hcint9Spec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - desc ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Hcint9Spec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 7 - desc TXERR"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<'_, Hcint9Spec> {
        TxerrW::new(self, 7)
    }
    #[doc = "Bit 8 - desc BBERR"]
    #[inline(always)]
    pub fn bberr(&mut self) -> BberrW<'_, Hcint9Spec> {
        BberrW::new(self, 8)
    }
    #[doc = "Bit 9 - desc FRMOR"]
    #[inline(always)]
    pub fn frmor(&mut self) -> FrmorW<'_, Hcint9Spec> {
        FrmorW::new(self, 9)
    }
    #[doc = "Bit 10 - desc DTERR"]
    #[inline(always)]
    pub fn dterr(&mut self) -> DterrW<'_, Hcint9Spec> {
        DterrW::new(self, 10)
    }
}
#[doc = "desc HCINT9\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcint9Spec;
impl crate::RegisterSpec for Hcint9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint9::R`](R) reader structure"]
impl crate::Readable for Hcint9Spec {}
#[doc = "`write(|w| ..)` method takes [`hcint9::W`](W) writer structure"]
impl crate::Writable for Hcint9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCINT9 to value 0"]
impl crate::Resettable for Hcint9Spec {}
