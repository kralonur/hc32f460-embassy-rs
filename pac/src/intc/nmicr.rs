#[doc = "Register `NMICR` reader"]
pub type R = crate::R<NmicrSpec>;
#[doc = "Register `NMICR` writer"]
pub type W = crate::W<NmicrSpec>;
#[doc = "Field `NMITRG` reader - desc NMITRG"]
pub type NmitrgR = crate::BitReader;
#[doc = "Field `NMITRG` writer - desc NMITRG"]
pub type NmitrgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSMPCLK` reader - desc NSMPCLK"]
pub type NsmpclkR = crate::FieldReader;
#[doc = "Field `NSMPCLK` writer - desc NSMPCLK"]
pub type NsmpclkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN` reader - desc NFEN"]
pub type NfenR = crate::BitReader;
#[doc = "Field `NFEN` writer - desc NFEN"]
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc NMITRG"]
    #[inline(always)]
    pub fn nmitrg(&self) -> NmitrgR {
        NmitrgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - desc NSMPCLK"]
    #[inline(always)]
    pub fn nsmpclk(&self) -> NsmpclkR {
        NsmpclkR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - desc NFEN"]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc NMITRG"]
    #[inline(always)]
    pub fn nmitrg(&mut self) -> NmitrgW<'_, NmicrSpec> {
        NmitrgW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc NSMPCLK"]
    #[inline(always)]
    pub fn nsmpclk(&mut self) -> NsmpclkW<'_, NmicrSpec> {
        NsmpclkW::new(self, 4)
    }
    #[doc = "Bit 7 - desc NFEN"]
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<'_, NmicrSpec> {
        NfenW::new(self, 7)
    }
}
#[doc = "desc NMICR\n\nYou can [`read`](crate::Reg::read) this register and get [`nmicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmicrSpec;
impl crate::RegisterSpec for NmicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmicr::R`](R) reader structure"]
impl crate::Readable for NmicrSpec {}
#[doc = "`write(|w| ..)` method takes [`nmicr::W`](W) writer structure"]
impl crate::Writable for NmicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMICR to value 0"]
impl crate::Resettable for NmicrSpec {}
