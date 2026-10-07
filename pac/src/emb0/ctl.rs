#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `PORTINEN` reader - desc PORTINEN"]
pub type PortinenR = crate::BitReader;
#[doc = "Field `PORTINEN` writer - desc PORTINEN"]
pub type PortinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN1` reader - desc CMPEN1"]
pub type Cmpen1R = crate::BitReader;
#[doc = "Field `CMPEN1` writer - desc CMPEN1"]
pub type Cmpen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN2` reader - desc CMPEN2"]
pub type Cmpen2R = crate::BitReader;
#[doc = "Field `CMPEN2` writer - desc CMPEN2"]
pub type Cmpen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEN3` reader - desc CMPEN3"]
pub type Cmpen3R = crate::BitReader;
#[doc = "Field `CMPEN3` writer - desc CMPEN3"]
pub type Cmpen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSCSTPEN` reader - desc OSCSTPEN"]
pub type OscstpenR = crate::BitReader;
#[doc = "Field `OSCSTPEN` writer - desc OSCSTPEN"]
pub type OscstpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN0` reader - desc PWMSEN0"]
pub type Pwmsen0R = crate::BitReader;
#[doc = "Field `PWMSEN0` writer - desc PWMSEN0"]
pub type Pwmsen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN1` reader - desc PWMSEN1"]
pub type Pwmsen1R = crate::BitReader;
#[doc = "Field `PWMSEN1` writer - desc PWMSEN1"]
pub type Pwmsen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSEN2` reader - desc PWMSEN2"]
pub type Pwmsen2R = crate::BitReader;
#[doc = "Field `PWMSEN2` writer - desc PWMSEN2"]
pub type Pwmsen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NFSEL` reader - desc NFSEL"]
pub type NfselR = crate::FieldReader;
#[doc = "Field `NFSEL` writer - desc NFSEL"]
pub type NfselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NFEN` reader - desc NFEN"]
pub type NfenR = crate::BitReader;
#[doc = "Field `NFEN` writer - desc NFEN"]
pub type NfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVSEL` reader - desc INVSEL"]
pub type InvselR = crate::BitReader;
#[doc = "Field `INVSEL` writer - desc INVSEL"]
pub type InvselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - desc PORTINEN"]
    #[inline(always)]
    pub fn portinen(&self) -> PortinenR {
        PortinenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc CMPEN1"]
    #[inline(always)]
    pub fn cmpen1(&self) -> Cmpen1R {
        Cmpen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc CMPEN2"]
    #[inline(always)]
    pub fn cmpen2(&self) -> Cmpen2R {
        Cmpen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc CMPEN3"]
    #[inline(always)]
    pub fn cmpen3(&self) -> Cmpen3R {
        Cmpen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - desc OSCSTPEN"]
    #[inline(always)]
    pub fn oscstpen(&self) -> OscstpenR {
        OscstpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - desc PWMSEN0"]
    #[inline(always)]
    pub fn pwmsen0(&self) -> Pwmsen0R {
        Pwmsen0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc PWMSEN1"]
    #[inline(always)]
    pub fn pwmsen1(&self) -> Pwmsen1R {
        Pwmsen1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - desc PWMSEN2"]
    #[inline(always)]
    pub fn pwmsen2(&self) -> Pwmsen2R {
        Pwmsen2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 28:29 - desc NFSEL"]
    #[inline(always)]
    pub fn nfsel(&self) -> NfselR {
        NfselR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - desc NFEN"]
    #[inline(always)]
    pub fn nfen(&self) -> NfenR {
        NfenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - desc INVSEL"]
    #[inline(always)]
    pub fn invsel(&self) -> InvselR {
        InvselR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - desc PORTINEN"]
    #[inline(always)]
    pub fn portinen(&mut self) -> PortinenW<'_, CtlSpec> {
        PortinenW::new(self, 0)
    }
    #[doc = "Bit 1 - desc CMPEN1"]
    #[inline(always)]
    pub fn cmpen1(&mut self) -> Cmpen1W<'_, CtlSpec> {
        Cmpen1W::new(self, 1)
    }
    #[doc = "Bit 2 - desc CMPEN2"]
    #[inline(always)]
    pub fn cmpen2(&mut self) -> Cmpen2W<'_, CtlSpec> {
        Cmpen2W::new(self, 2)
    }
    #[doc = "Bit 3 - desc CMPEN3"]
    #[inline(always)]
    pub fn cmpen3(&mut self) -> Cmpen3W<'_, CtlSpec> {
        Cmpen3W::new(self, 3)
    }
    #[doc = "Bit 5 - desc OSCSTPEN"]
    #[inline(always)]
    pub fn oscstpen(&mut self) -> OscstpenW<'_, CtlSpec> {
        OscstpenW::new(self, 5)
    }
    #[doc = "Bit 6 - desc PWMSEN0"]
    #[inline(always)]
    pub fn pwmsen0(&mut self) -> Pwmsen0W<'_, CtlSpec> {
        Pwmsen0W::new(self, 6)
    }
    #[doc = "Bit 7 - desc PWMSEN1"]
    #[inline(always)]
    pub fn pwmsen1(&mut self) -> Pwmsen1W<'_, CtlSpec> {
        Pwmsen1W::new(self, 7)
    }
    #[doc = "Bit 8 - desc PWMSEN2"]
    #[inline(always)]
    pub fn pwmsen2(&mut self) -> Pwmsen2W<'_, CtlSpec> {
        Pwmsen2W::new(self, 8)
    }
    #[doc = "Bits 28:29 - desc NFSEL"]
    #[inline(always)]
    pub fn nfsel(&mut self) -> NfselW<'_, CtlSpec> {
        NfselW::new(self, 28)
    }
    #[doc = "Bit 30 - desc NFEN"]
    #[inline(always)]
    pub fn nfen(&mut self) -> NfenW<'_, CtlSpec> {
        NfenW::new(self, 30)
    }
    #[doc = "Bit 31 - desc INVSEL"]
    #[inline(always)]
    pub fn invsel(&mut self) -> InvselW<'_, CtlSpec> {
        InvselW::new(self, 31)
    }
}
#[doc = "desc CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {}
