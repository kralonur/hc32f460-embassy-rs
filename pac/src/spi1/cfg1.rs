#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `FTHLV` reader - desc FTHLV"]
pub type FthlvR = crate::FieldReader;
#[doc = "Field `FTHLV` writer - desc FTHLV"]
pub type FthlvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SPRDTD` reader - desc SPRDTD"]
pub type SprdtdR = crate::BitReader;
#[doc = "Field `SPRDTD` writer - desc SPRDTD"]
pub type SprdtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS0PV` reader - desc SS0PV"]
pub type Ss0pvR = crate::BitReader;
#[doc = "Field `SS0PV` writer - desc SS0PV"]
pub type Ss0pvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS1PV` reader - desc SS1PV"]
pub type Ss1pvR = crate::BitReader;
#[doc = "Field `SS1PV` writer - desc SS1PV"]
pub type Ss1pvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS2PV` reader - desc SS2PV"]
pub type Ss2pvR = crate::BitReader;
#[doc = "Field `SS2PV` writer - desc SS2PV"]
pub type Ss2pvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS3PV` reader - desc SS3PV"]
pub type Ss3pvR = crate::BitReader;
#[doc = "Field `SS3PV` writer - desc SS3PV"]
pub type Ss3pvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSSI` reader - desc MSSI"]
pub type MssiR = crate::FieldReader;
#[doc = "Field `MSSI` writer - desc MSSI"]
pub type MssiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSSDL` reader - desc MSSDL"]
pub type MssdlR = crate::FieldReader;
#[doc = "Field `MSSDL` writer - desc MSSDL"]
pub type MssdlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MIDI` reader - desc MIDI"]
pub type MidiR = crate::FieldReader;
#[doc = "Field `MIDI` writer - desc MIDI"]
pub type MidiW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - desc FTHLV"]
    #[inline(always)]
    pub fn fthlv(&self) -> FthlvR {
        FthlvR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 6 - desc SPRDTD"]
    #[inline(always)]
    pub fn sprdtd(&self) -> SprdtdR {
        SprdtdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - desc SS0PV"]
    #[inline(always)]
    pub fn ss0pv(&self) -> Ss0pvR {
        Ss0pvR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - desc SS1PV"]
    #[inline(always)]
    pub fn ss1pv(&self) -> Ss1pvR {
        Ss1pvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - desc SS2PV"]
    #[inline(always)]
    pub fn ss2pv(&self) -> Ss2pvR {
        Ss2pvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - desc SS3PV"]
    #[inline(always)]
    pub fn ss3pv(&self) -> Ss3pvR {
        Ss3pvR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 20:22 - desc MSSI"]
    #[inline(always)]
    pub fn mssi(&self) -> MssiR {
        MssiR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - desc MSSDL"]
    #[inline(always)]
    pub fn mssdl(&self) -> MssdlR {
        MssdlR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - desc MIDI"]
    #[inline(always)]
    pub fn midi(&self) -> MidiR {
        MidiR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc FTHLV"]
    #[inline(always)]
    pub fn fthlv(&mut self) -> FthlvW<'_, Cfg1Spec> {
        FthlvW::new(self, 0)
    }
    #[doc = "Bit 6 - desc SPRDTD"]
    #[inline(always)]
    pub fn sprdtd(&mut self) -> SprdtdW<'_, Cfg1Spec> {
        SprdtdW::new(self, 6)
    }
    #[doc = "Bit 8 - desc SS0PV"]
    #[inline(always)]
    pub fn ss0pv(&mut self) -> Ss0pvW<'_, Cfg1Spec> {
        Ss0pvW::new(self, 8)
    }
    #[doc = "Bit 9 - desc SS1PV"]
    #[inline(always)]
    pub fn ss1pv(&mut self) -> Ss1pvW<'_, Cfg1Spec> {
        Ss1pvW::new(self, 9)
    }
    #[doc = "Bit 10 - desc SS2PV"]
    #[inline(always)]
    pub fn ss2pv(&mut self) -> Ss2pvW<'_, Cfg1Spec> {
        Ss2pvW::new(self, 10)
    }
    #[doc = "Bit 11 - desc SS3PV"]
    #[inline(always)]
    pub fn ss3pv(&mut self) -> Ss3pvW<'_, Cfg1Spec> {
        Ss3pvW::new(self, 11)
    }
    #[doc = "Bits 20:22 - desc MSSI"]
    #[inline(always)]
    pub fn mssi(&mut self) -> MssiW<'_, Cfg1Spec> {
        MssiW::new(self, 20)
    }
    #[doc = "Bits 24:26 - desc MSSDL"]
    #[inline(always)]
    pub fn mssdl(&mut self) -> MssdlW<'_, Cfg1Spec> {
        MssdlW::new(self, 24)
    }
    #[doc = "Bits 28:30 - desc MIDI"]
    #[inline(always)]
    pub fn midi(&mut self) -> MidiW<'_, Cfg1Spec> {
        MidiW::new(self, 28)
    }
}
#[doc = "desc CFG1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0x10"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0x10;
}
